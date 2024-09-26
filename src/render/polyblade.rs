/*

use iced::alignment::{Horizontal, Vertical};
use iced::mouse;
use iced::widget::{button, Button, Row};
use iced::Rectangle;
use std::io::Read;

use iced::widget::slider;
use iced_aw::menu_bar;
use iced_aw::{helpers::color_picker, menu::Item};

use crate::render::{
    menu::*,
    message::*,
    pipeline::PolyhedronPrimitive,
    polydex::{Entry, Polydex},
    state::AppState,
};
use iced::widget::text;
use iced::{
    executor, font,
    widget::{column, container, row, shader},
    window, Application, Element, Length, Subscription, Theme,
};

pub struct Polyblade {
    state: AppState,
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
    type Message = PolybladeMessage;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Polyblade")
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: AppState::default(),
            },
            Command::batch(vec![
                // font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(PolybladeMessage::FontLoaded),
                // font::load(iced_aw::NERD_FONT_BYTES).map(PolybladeMessage::FontLoaded),
                Command::perform(async { load_polydex() }, |polydex| {
                    PolybladeMessage::PolydexLoaded(polydex.map_err(|err| err.to_string()))
                }),
            ]),
        )
    }

    fn subscription(&self) -> Subscription<PolybladeMessage> {
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
                pm.map(PolybladeMessage::Preset)
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
                cm.map(PolybladeMessage::Conway)
            }
        };

        let tick = window::frames().map(PolybladeMessage::Tick);

        if self.state.render.picker.color_index.is_some() {
            Subscription::batch(vec![keyboard::on_key_press(handle_hotkey)])
        } else {
            Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}

*/
