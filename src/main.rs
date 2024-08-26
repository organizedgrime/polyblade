mod bones;
mod render;
use iced::Application as _;
use render::Polyblade;

fn main() -> iced::Result {
    Polyblade::run(iced::Settings::default())
}
