// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Polyblade example
//!
//! Demonstrates use of a custom draw pipe.
//!

extern crate chrono;

mod color;
mod message;
mod pipeline;
mod polyhedra;

use chrono::prelude::*;
use message::*;
use std::time::Duration;
use strum::IntoEnumIterator;

use color::*;
use kas::dir::Right;
use pipeline::*;
use polyhedra::*;

use kas::draw::{Draw, DrawIface};
use kas::event::{self};
use kas::prelude::*;
use kas::widgets::{adapt, menu, Adapt, CheckButton, Separator, Slider};
use kas_wgpu::draw::{DrawCustom, DrawPipe};

type Key = kas::event::Key<kas::event::SmolStr>;

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

        fn handle(&mut self, key: Key) {
            println!("key pressed: {key:?}");
            match key {
                Key::Character(s) if s == "+" => {
                    println!("+ pressed")
                },
                Key::Character(s) if s == "o" => {
                    println!("o pressed")
                },
                _ => (),
            }
        }
    }

    impl Layout for Polyblade {
        fn size_rules(&mut self, sizer: SizeCx, axis: AxisInfo) -> SizeRules {
            kas::layout::LogicalSize(400.0, 400.0)
                .to_rules_with_factor(axis, sizer.scale_factor(), 4.0)
                .with_stretch(Stretch::Low)
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

        fn update(&mut self, _: &mut ConfigCx, _: &AppData) {}

        fn handle_event(&mut self, cx: &mut EventCx, _: &AppData, event: Event) -> IsUsed {
            println!("nya: {event:?}");
            match event {
                // TODO Orbital controls
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
    now: DateTime<Local>,
    disabled: bool,
}

fn widgets() -> Box<dyn Widget<Data = AppData>> {
    #[derive(Clone, Debug)]
    enum Item {
        Button,
        Check(bool),
        Slider(i32),
    }

    impl_scope! {
        #[derive(Debug)]
        #[impl_default]
        struct Data {
            check: bool = true,
            value: i32 = 5,
        }
    }
    let data = Data {
        ..Default::default()
    };

    let widgets = kas::aligned_column![
        row![
            "CheckButton",
            CheckButton::new_msg("&Check me", |_, data: &Data| data.check, Item::Check)
        ],
        row![
            "Slider",
            Slider::right(0..=10, |_, data: &Data| data.value).with_msg(Item::Slider)
        ],
    ];

    let ui = Adapt::new(widgets, data).on_message(|cx, data, item| {
        println!("Message: {item:?}");
        match item {
            Item::Check(v) => data.check = v,
            _ => (),
        }
    });

    let ui = adapt::AdaptEvents::new(ui)
        .on_update(|cx, _, data: &AppData| cx.set_disabled(data.disabled));

    Box::new(ui)
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
        Quit,
        Preset(PresetMenu),
    }

    let menubar = menu::MenuBar::<AppData, Right>::builder()
        .menu("&App", |menu| {
            menu.entry("&Quit", Menu::Quit);
        })
        .menu("&Preset", |menu| {
            let _: menu::SubMenuBuilder<AppData> =
                PresetMenu::iter().fold(menu, |menu, menu_item| match menu_item {
                    PresetMenu::Prism(_) => menu.submenu("Prism", |mut menu| {
                        for i in 3..=8 {
                            let entry = PresetMenu::Prism(i);
                            menu.push_entry(entry.to_string(), entry);
                        }
                    }),
                    PresetMenu::AntiPrism(_) => menu.submenu("AntiPrism", |mut menu| {
                        for i in 2..=8 {
                            let entry = PresetMenu::AntiPrism(i);
                            menu.push_entry(entry.to_string(), entry);
                        }
                    }),
                    PresetMenu::Pyramid(_) => menu.submenu("Pyramid", |mut menu| {
                        for i in 3..=8 {
                            let entry = PresetMenu::Pyramid(i);
                            menu.push_entry(entry.to_string(), entry);
                        }
                    }),
                    _ => menu.entry(menu_item.to_string(), Menu::Preset(menu_item)),
                });
        })
        .build();

    let ui = impl_anon! {
        #[widget{
            layout = column! [
                self.menubar,
                Separator::new(),
                self.pblade,
                Separator::new(),
                self.widgets
            ];
        }]
        struct {
            core: widget_core!(),
            state: AppData,
            #[widget(&self.state)] menubar: menu::MenuBar::<AppData, Right> = menubar,
            #[widget(&self.state)] pblade: Polyblade = Polyblade::new(),
            #[widget(&self.state)] widgets: Box<dyn Widget<Data = AppData>> = widgets(),
        }
        impl Events for Self {
            type Data = ();

            fn configure(&mut self, cx: &mut ConfigCx) {
                cx.request_timer(self.id(), 0, Duration::new(0, 0))
            }

            fn handle_messages(&mut self, cx: &mut EventCx, _: &Self::Data) {
                if let Some(msg) = cx.try_pop::<Menu>() {
                    match msg {
                        Menu::Quit => {
                            cx.exit();
                        }
                        Menu::Preset(preset) => {
                            self.pblade.polyhedron = preset.polyhedron();
                        }
                        _ => {}
                    }
                }
            }

            fn handle_event(&mut self, cx: &mut EventCx, _: &Self::Data,  event: Event) -> IsUsed {
                match event {
                    Event::Timer(0) => {
                        self.state.now = Local::now();
                        self.pblade.polyhedron.update();
                        // Locked at 60fps
                        let ns = (1_000_000_000 - (self.state.now.time().nanosecond() % 1_000_000_000)) / 60;
                        cx.request_timer(self.id(), 0, Duration::new(0, ns));
                        cx.redraw(self);
                        Used
                    }
                    _ => Unused,
                }

            }
        }
    }
    .on_message(|_, polyblade, key| polyblade.pblade.handle(key))
    .on_configure(|cx, _| {
        cx.disable_nav_focus(true);
        cx.enable_alt_bypass(true);
    });

    app.add(Window::new(ui, "Polyblade"));
    app.run()
}
