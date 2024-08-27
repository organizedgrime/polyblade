use std::io::Read;

use iced::mouse;
use iced::widget::button;
use iced::Rectangle;
use ultraviolet::Vec3;

use iced::widget::slider;
use iced_aw::menu::Item;
use iced_aw::menu_bar;

use crate::{
    bones::Transaction,
    render::{
        menu::*,
        message::*,
        pipeline::PolyhedronPrimitive,
        polydex::{Entry, Polydex},
        state::AppState,
    },
    Instant,
};
use iced::widget::{checkbox, text};
use iced::{
    executor, font,
    widget::{column, container, row, shader},
    window, Application, Command, Element, Length, Subscription, Theme,
};

pub struct Polyblade {
    state: AppState,
    polydex: Polydex,
}

pub fn load_polydex() -> Result<Polydex, Box<dyn std::error::Error>> {
    let mut polydex = std::fs::File::open("polydex.ron")?;
    let mut polydex_str = String::new();
    polydex.read_to_string(&mut polydex_str)?;
    let polydex: Vec<Entry> = ron::from_str(&polydex_str).map_err(|_| "Ron parsing error")?;
    Ok(polydex)
}

impl Application for Polyblade {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Polyblade")
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: AppState::default(),
                polydex: vec![],
            },
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(async { load_polydex() }, |polydex| {
                    Message::PolydexLoaded(polydex.map_err(|err| err.to_string()))
                }),
            ]),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;
        match message {
            FontLoaded(_) => {}
            PolydexLoaded(polydex) => {
                if let Ok(polydex) = polydex {
                    self.polydex = polydex;
                    self.state.info = self.state.polyhedron.polydex_entry(&self.polydex);
                } else {
                    //tracing_subscriber::warn
                }
            }
            Tick(time) => {
                if self.state.schlegel {
                    self.state.camera.eye = self.state.polyhedron.face_centroid(0) * 1.1;
                }

                // If the polyhedron has changed
                if self.state.info.conway != self.state.polyhedron.name {
                    // Recompute its Polydex entry
                    self.state.info = self.state.polyhedron.polydex_entry(&self.polydex);
                }

                self.state.update(time);
            }
            Rotate(rotating) => {
                self.state.rotating = rotating;
                if !rotating {
                    self.state.rotation_duration = Instant::now().duration_since(self.state.start);
                } else {
                    self.state.start = Instant::now()
                        .checked_sub(self.state.rotation_duration)
                        .unwrap();
                }
            }
            Schlegel(schlegel) => {
                self.state.schlegel = schlegel;
                if schlegel {
                    self.state.camera.fov_y = std::f32::consts::PI * 0.962;
                } else {
                    self.state.camera.fov_y = 1.0;
                    self.state.camera.eye = Vec3::new(0.0, 2.0, 3.0);
                }
            }
            SizeChanged(size) => {
                self.state.scale = size;
            }
            FovChanged(fov) => {
                self.state.camera.fov_y = fov;
            }
            Preset(preset) => self.state.polyhedron.change_shape(preset),
            Conway(conway) => {
                self.state
                    .polyhedron
                    .transactions
                    .push(Transaction::Conway(conway));
            }
            OpenWiki(wiki) => {
                let _ = open::that(wiki).ok();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            column![
                row![
                    menu_bar!((bar("Preset"), PresetMessage::menu())(
                        bar("Conway"),
                        ConwayMessage::menu()
                    )),
                    checkbox("Rotating", self.state.rotating).on_toggle(Message::Rotate),
                    checkbox("Schlegel Diagram", self.state.schlegel).on_toggle(Message::Schlegel)
                ]
                .spacing(10.0),
                // Actual shader of the program
                shader(self).width(Length::Fill).height(Length::Fill),
                // Info
                container(column![
                    button(text(self.state.info.name())).on_press(self.state.info.wiki_message()),
                    row![
                        column![
                            text("Bowers:"),
                            text("Conway:"),
                            text("Faces:"),
                            text("Edges:"),
                            text("Vertices:"),
                        ],
                        column![
                            text(self.state.info.bowers()),
                            text(&self.state.info.conway),
                            text(&self.state.info.faces),
                            text(&self.state.info.edges),
                            text(&self.state.info.vertices),
                        ]
                    ]
                    .spacing(20)
                ]),
                row![
                    text("Size: "),
                    text(self.state.scale.to_string()),
                    slider(1.0..=10.0, self.state.scale, Message::SizeChanged).step(0.1)
                ],
                row![
                    text("FOV: "),
                    text(self.state.camera.fov_y.to_string()),
                    slider(
                        0.0..=(std::f32::consts::PI),
                        self.state.camera.fov_y,
                        Message::FovChanged
                    )
                    .step(0.1)
                ]
            ]
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(10)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        use iced::keyboard;
        use keyboard::key;
        let handle_hotkey = |key: key::Key, modifiers: keyboard::Modifiers| {
            use keyboard::Key::Character;
            if modifiers.command() {
                use PresetMessage::*;
                let pm = match key.as_ref() {
                    Character("t") => Some(Pyramid(3)),
                    Character("c") => Some(Prism(4)),
                    Character("o") => Some(Octahedron),
                    Character("d") => Some(Dodecahedron),
                    Character("i") => Some(Icosahedron),
                    _ => None,
                };
                pm.map(Message::Preset)
            } else {
                use ConwayMessage::*;
                let cm = match key.as_ref() {
                    Character("d") => Some(Dual),
                    Character("e") => Some(Expand),
                    Character("s") => Some(Snub),
                    Character("k") => Some(Kis),
                    Character("j") => Some(Join),
                    Character("a") => Some(Ambo),
                    Character("t") => Some(Truncate),
                    Character("b") => Some(Bevel),
                    _ => None,
                };
                cm.map(Message::Conway)
            }
        };

        let tick = window::frames().map(Message::Tick);

        Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
    }

    fn theme(&self) -> Self::Theme {
        Theme::KanagawaLotus
    }
}

impl<Message> shader::Program<Message> for Polyblade {
    type State = ();
    type Primitive = PolyhedronPrimitive;

    /* fn update(
        &self,
        _state: &mut Self::State,
        event: shader::Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) -> (event::Status, Option<Message>) {
        match event {
            /* shader::Event::Mouse(_) => {}
            shader::Event::Touch(_) => {}
            shader::Event::Keyboard(_) => {} */
            shader::Event::RedrawRequested(time) => {
                println!("redraw requested11");
                (event::Status::Captured, None)
            }
            _ => (event::Status::Ignored, None),
        }
    } */

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        Self::Primitive::new(
            self.state.polyhedron.clone(),
            self.state.schlegel,
            self.state.palette.clone(),
            self.state.transform,
            self.state.camera,
        )
    }
}
