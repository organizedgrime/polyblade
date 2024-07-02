mod menu;
mod message;
mod polyhedra;
mod scene;

use std::time::Duration;

use glam::vec3;
use iced::widget::slider;
use iced_aw::menu::Item;
use iced_aw::menu_bar;
use menu::*;
use message::*;
use polyhedra::Transaction;
use scene::Scene;

use iced::widget::{checkbox, shader::wgpu, text};
use iced::{
    executor, font,
    time::Instant,
    widget::{column, container, row, shader},
    window, Application, Command, Element, Length, Subscription, Theme,
};

fn main() -> iced::Result {
    Polyblade::run(iced::Settings::default())
}

struct Polyblade {
    start: Instant,
    scene: Scene,
    rotating: bool,
    schlegel: bool,
    rotation_duration: Duration,
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
                start: Instant::now(),
                scene: Scene::new(),
                rotating: true,
                schlegel: false,
                rotation_duration: Duration::from_secs(0),
            },
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;
        match message {
            FontLoaded(_) => {}
            Tick(time) => {
                if self.schlegel {
                    self.scene.camera.eye = self.scene.polyhedron.face_centroid(0) * 1.1;
                    self.scene.clear_face = Some(0);
                }

                if self.rotating {
                    self.scene
                        .update(self.schlegel, time.duration_since(self.start));
                } else {
                    self.scene.update(self.schlegel, self.rotation_duration);
                }
            }
            Rotate(rotating) => {
                self.rotating = rotating;
                if !rotating {
                    self.rotation_duration = Instant::now().duration_since(self.start);
                } else {
                    self.start = Instant::now().checked_sub(self.rotation_duration).unwrap();
                }
            }
            Schlegel(schlegel) => {
                self.schlegel = schlegel;
                if schlegel {
                    self.scene.camera.fov_y = std::f32::consts::PI * 0.962;
                } else {
                    self.scene.camera.fov_y = 1.0;
                    self.scene.camera.eye = vec3(0.0, 2.0, 3.0);
                    self.scene.clear_face = None;
                }
            }
            SizeChanged(size) => {
                self.scene.size = size;
            }
            FovChanged(fov) => {
                self.scene.camera.fov_y = fov;
            }
            Preset(preset) => self.scene.polyhedron.change_shape(preset),
            Conway(conway) => {
                self.scene
                    .polyhedron
                    .transactions
                    .push(Transaction::Conway(conway));
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
                    /*
                    .style(|theme: &Theme| {
                        menu::Appearance {
                            bar_background: Color::WHITE.into(),
                            menu_background: Color::WHITE.into(),
                            ..theme.appearance(&MenuBarStyle::Default)
                        }
                    }),
                    */
                    checkbox("Rotating", self.rotating).on_toggle(Message::Rotate),
                    checkbox("Schlegel Diagram", self.schlegel).on_toggle(Message::Schlegel)
                ]
                .spacing(10.0),
                // Actual shader of the program
                shader(&self.scene).width(Length::Fill).height(Length::Fill),
                // Info
                container(
                    row![
                        column![
                            text("Conway:"),
                            text("Faces:"),
                            text("Edges:"),
                            text("Vertices:"),
                        ],
                        column![
                            text(&self.scene.polyhedron.name),
                            text(self.scene.polyhedron.cycles.len().to_string()),
                            text(self.scene.polyhedron.edges.len().to_string(),),
                            text(self.scene.polyhedron.vertices.len().to_string())
                        ]
                    ]
                    .spacing(20)
                ),
                row![
                    text("Size: "),
                    text(self.scene.size.to_string()),
                    slider(1.0..=10.0, self.scene.size, Message::SizeChanged).step(0.1)
                ],
                row![
                    text("FOV: "),
                    text(self.scene.camera.fov_y.to_string()),
                    slider(
                        0.0..=(std::f32::consts::PI),
                        self.scene.camera.fov_y,
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
