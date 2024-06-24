mod message;
mod polyhedra;
mod scene;

use std::time::Duration;

use glam::vec3;
use iced::Color;
use iced_aw::color_picker;
use iced_aw::menu::{Item, MenuBar};
use message::*;
use polyhedra::Transaction;
use scene::Scene;

use iced::widget::{checkbox, shader::wgpu, text};
use iced::{
    executor, font,
    time::Instant,
    widget::{button, column, container, row, shader},
    window, Application, Command, Element, Length, Subscription, Theme,
};

fn main() -> iced::Result {
    Polyblade::run(iced::Settings::default())
}

struct Polyblade {
    start: Instant,
    scene: Scene,
    rotating: bool,
    rotation_duration: Duration,
    show_picker: bool,
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
                rotation_duration: Duration::from_secs(0),
                show_picker: false,
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
                if self.rotating {
                    self.scene.update(time.duration_since(self.start));
                } else {
                    self.scene.update(self.rotation_duration);
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
                    MenuBar::new(vec![
                        Item::with_menu(button(text("Preset")), PresetMessage::menu()),
                        Item::with_menu(button(text("Conway")), ConwayMessage::menu())
                    ]),
                    checkbox("Rotating", self.rotating).on_toggle(Message::Rotate)
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
        let handle_hotkey = |key: key::Key, _modifiers: keyboard::Modifiers| match key.as_ref() {
            keyboard::Key::Character("d") => Some(Message::Conway(ConwayMessage::Dual)),
            keyboard::Key::Character("e") => Some(Message::Conway(ConwayMessage::Expand)),
            keyboard::Key::Character("s") => Some(Message::Conway(ConwayMessage::Snub)),
            keyboard::Key::Character("k") => Some(Message::Conway(ConwayMessage::Kis)),
            keyboard::Key::Character("j") => Some(Message::Conway(ConwayMessage::Join)),
            keyboard::Key::Character("c") => Some(Message::Conway(ConwayMessage::Contract)),
            keyboard::Key::Character("a") => Some(Message::Conway(ConwayMessage::Ambo)),
            keyboard::Key::Character("t") => Some(Message::Conway(ConwayMessage::Truncate)),
            keyboard::Key::Character("b") => Some(Message::Conway(ConwayMessage::Bevel)),
            _ => None,
        };
        let tick = window::frames().map(Message::Tick);

        Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
    }

    fn theme(&self) -> Self::Theme {
        Theme::KanagawaLotus
    }
}
