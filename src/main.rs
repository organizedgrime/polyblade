mod color;
mod menu;
mod message;
mod polyhedra;
mod scene;

use ultraviolet::Vec3;

use color::*;
use iced::widget::slider;
use iced_aw::menu::Item;
use iced_aw::menu_bar;
use menu::*;
use message::*;
use polyhedra::Transaction;
use scene::{AppState, Scene};

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
    scene: Scene,
    state: AppState,
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
                scene: Scene::default(),
                state: AppState::default(),
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
                if self.state.schlegel {
                    self.state.camera.eye = self.state.polyhedron.face_centroid(0) * 1.1;
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
                            text(&self.state.polyhedron.name),
                            text(self.state.polyhedron.cycles.len().to_string()),
                            text(self.state.polyhedron.edges.len().to_string(),),
                            text(self.state.polyhedron.vertices.len().to_string())
                        ]
                    ]
                    .spacing(20)
                ),
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
