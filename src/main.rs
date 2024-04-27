mod scene;

use scene::Scene;

use iced::executor;
use iced::time::Instant;
use iced::widget::shader::wgpu;
use iced::widget::{checkbox, column, container, row, shader, slider, text};
use iced::window;
use iced::{Alignment, Application, Color, Command, Element, Length, Subscription, Theme};

fn main() -> iced::Result {
    Polyblade::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
}

struct Polyblade {
    start: Instant,
    scene: Scene,
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
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick(time) => {
                self.scene.update(time - self.start);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let shader = shader(&self.scene).width(Length::Fill).height(Length::Fill);

        container(column![shader].align_items(Alignment::Center))
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
