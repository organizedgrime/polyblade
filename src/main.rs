mod message;
mod scene;
use iced::alignment::{Horizontal, Vertical};
use iced_aw::{card, modal, Card};
use message::*;
use scene::Scene;

use iced::executor;
use iced::time::Instant;
use iced::widget::shader::wgpu;
use iced::widget::{
    button, checkbox, column, container, row, shader, slider, text, Button, Row, Text,
};
use iced::window;
use iced::{Alignment, Application, Color, Command, Element, Length, Subscription, Theme};
use strum::IntoEnumIterator;

fn main() -> iced::Result {
    Polyblade::run(iced::Settings::default())
}

struct Polyblade {
    start: Instant,
    scene: Scene,
    rotating: bool,
    show_alert: bool,
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
                show_alert: false,
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick(time) => {
                self.scene.update(time - self.start);
            }
            Message::Rotate(rotating) => {
                self.rotating = rotating;
            }
            Message::CloseAlert => {
                self.show_alert = false;
            }
            Message::Conway(_) => {
                self.show_alert = true;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        //let conway_buttons =             .collect();
        let conway_row = Row::with_children(ConwayMessage::iter().map(|message| {
            button(Text::new(message.to_string()))
                .on_press(Message::Conway(message))
                .into()
        }))
        .spacing(10);

        let underlay = container(
            column![
                shader(&self.scene).width(Length::Fill).height(Length::Fill),
                checkbox("Rotating", self.rotating).on_toggle(Message::Rotate),
                conway_row
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(10);

        let card = if self.show_alert {
            Some(
                card("Error", "Sorry, that isn't implemented yet.")
                    .foot(
                        row![button("Ok")
                            .width(Length::Fill)
                            .on_press(Message::CloseAlert)]
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill),
                    )
                    .max_width(300.0)
                    .on_close(Message::CloseAlert),
            )
        } else {
            None
        };

        modal(underlay, card)
            .backdrop(Message::CloseAlert)
            .on_esc(Message::CloseAlert)
            //.align_y(Vertical::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}
