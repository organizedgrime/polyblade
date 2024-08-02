// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Polyblade example
//!
//! Demonstrates use of a custom draw pipe.
//!

mod polyhedra;
use kas::dir::Right;
use polyhedra::*;
mod color;
use color::*;
mod pipeline;
use pipeline::*;

use kas::draw::{Draw, DrawIface};
use kas::event::{self, Command, Key};
use kas::prelude::*;
use kas::widgets::{menu, Separator};
use kas_wgpu::draw::{DrawCustom, DrawPipe};

#[derive(Clone, Debug)]
pub struct ViewUpdate;

impl_scope! {
    #[widget]
    #[derive(Clone)]
    pub struct Polyblade {
        core: widget_core!(),
        polyhedron: PolyGraph,
        size: f32,
    }

    impl Polyblade {
        fn new() -> Self {
            Polyblade {
                core: Default::default(),
                polyhedron: PolyGraph::icosahedron(),
                size: 1.0,
            }
        }

        fn reset_view(&mut self) {
            self.size = 1.0;
        }
    }

    impl Layout for Polyblade {
        fn size_rules(&mut self, sizer: SizeCx, axis: AxisInfo) -> SizeRules {
            kas::layout::LogicalSize(800.0, 800.0)
                .to_rules_with_factor(axis, sizer.scale_factor(), 4.0)
                .with_stretch(Stretch::High)
        }

        #[inline]
        fn set_rect(&mut self, _: &mut ConfigCx, rect: Rect) {
            self.core.rect = rect;
        }

        fn draw(&mut self, mut draw: DrawCx) {
            let draw = draw.draw_device();
            let draw = DrawIface::<DrawPipe<Pipe>>::downcast_from(draw).unwrap();
            draw.draw.custom(draw.get_pass(), self.core.rect, self.clone());
        }
    }

    impl Events for Polyblade {
        type Data = AppData;

        fn configure(&mut self, cx: &mut ConfigCx) {
            cx.register_nav_fallback(self.id());
        }

        fn update(&mut self, _: &mut ConfigCx, data: &AppData) {
            self.polyhedron.update();
        }

        fn navigable(&self) -> bool {
            true
        }

        fn handle_event(&mut self, cx: &mut EventCx, _: &AppData, event: Event) -> IsUsed {
            println!("nya: {event:?}");
            match event {
                Event::Key(event, is_synthetic) => {
                    println!("key: {event:?}");
                    let s: Key = Key::Character("s".into());
                    match event.logical_key {
                        s => {
                            self.polyhedron = PolyGraph::dodecahedron();
                        },
                        _ => {}
                    }
                }
                Event::Command(cmd, _) => {
                    match cmd {
                        Command::Home | Command::End => self.reset_view(),
                        Command::PageUp => {},
                        Command::PageDown => {},
                        cmd => {
                        }
                    }
                    cx.push(ViewUpdate);
                }
                Event::Scroll(delta) => {
                    let factor = match delta {
                        event::ScrollDelta::LineDelta(_, y) => -0.5 * y as f64,
                        event::ScrollDelta::PixelDelta(coord) => -0.01 * coord.1 as f64,
                    };
                    cx.push(ViewUpdate);
                }
                Event::Pan { alpha, delta } => {
                    cx.push(ViewUpdate);
                }
                Event::PressStart { press } => {
                    return press.grab(self.id())
                        .with_mode(event::GrabMode::PanFull)
                        .with_icon(event::CursorIcon::Grabbing)
                        .with_cx(cx);
                }
                _ => return Unused,
            }
            Used
        }
    }
}

#[derive(Debug, Default)]
pub struct AppData {
    disabled: bool,
}
fn main() -> kas::app::Result<()> {
    env_logger::init();

    //let window = Window::new(PolybladeUI::new(), "Polyblade");
    let theme = kas::theme::FlatTheme::new().with_colours("light");
    let mut app = kas::app::WgpuBuilder::new(PipeBuilder)
        .with_theme(theme)
        .build(())?;

    #[derive(Clone, Debug)]
    enum Menu {
        Theme(&'static str),
        Colour(String),
        Disabled(bool),
        Quit,
    }

    let menubar = menu::MenuBar::<AppData, Right>::builder()
        .menu("&App", |menu| {
            menu.entry("&Quit", Menu::Quit);
        })
        .build();

    let ui = impl_anon! {
        #[widget{
            layout = column! [
                self.menubar,
                Separator::new(),
                self.pblade
            ];
        }]
        struct {
            core: widget_core!(),
            state: AppData,
            #[widget(&self.state)] menubar: menu::MenuBar::<AppData, Right> = menubar,
            #[widget(&self.state)] pblade: Polyblade = Polyblade::new(),
        }
        impl Events for Self {
            type Data = ();

            fn handle_messages(&mut self, cx: &mut EventCx, _: &Self::Data) {
                if let Some(msg) = cx.try_pop::<Menu>() {
                    match msg {
                        Menu::Quit => {
                            cx.exit();
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    app.add(Window::new(ui, "Polyblade"));
    app.run()
}
