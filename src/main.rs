mod scene;

use scene::Scene;

use iced::executor;
use iced::time::Instant;
use iced::widget::shader::wgpu;
use iced::widget::{button, checkbox, column, container, row, shader, slider, text};
use iced::window;
use iced::{Alignment, Application, Color, Command, Element, Length, Subscription, Theme};

fn main() -> iced::Result {
    Polyblade::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum ConwayMessage {
    // 1
    Seed,
    Dual,
    // 2
    Join,
    Ambo,
    // 3
    Kis,
    Needle,
    Zip,
    Truncate,
    // 4
    Ortho,
    Expand,
    // 5
    Gyro,
    Snub,
    // 6
    Meta,
    Bevel,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
    Rotate(bool),
    Conway(ConwayMessage),
}

struct Polyblade {
    start: Instant,
    scene: Scene,
    rotating: bool,
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
            Message::Conway(conway) => {
                println!("unimplemented");
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let shader = shader(&self.scene).width(Length::Fill).height(Length::Fill);

        let ui = column![
            //row![checkbox("Rotating", self.rotating).on_toggle(Message::Rotate)].padding(10)
            row![
                checkbox("Rotating", self.rotating).on_toggle(Message::Rotate),
                button("").on_press(Message::Conway(ConwayMessage::Kis))
            ]
            .padding(10)
        ]
        .padding(10);

        container(column![shader, ui].align_items(Alignment::Center))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}
