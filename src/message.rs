use std::time::Instant;

use iced::{
    alignment, font,
    widget::{button, row, text},
    Element, Length, Renderer, Theme,
};
use iced_aw::{
    menu::{Item, Menu},
    BootstrapIcon, BOOTSTRAP_FONT,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub trait MenuAble {
    fn bar<'a>(label: &str) -> button::Button<'a, Message, Theme, Renderer> {
        button(text(label).vertical_alignment(alignment::Vertical::Center)).width(Length::Shrink)
    }

    fn base<'a>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        msg: Message,
    ) -> button::Button<'a, Message, Theme, Renderer> {
        button(content).padding([4, 8]).on_press(msg)
    }

    fn labeled<'a>(label: &str, msg: Message) -> button::Button<'a, Message, Theme, Renderer> {
        Self::base(
            text(label).vertical_alignment(alignment::Vertical::Center),
            msg,
        )
        .width(Length::Fill)
    }

    fn base_menu(
        items: Vec<Item<'_, Message, Theme, Renderer>>,
    ) -> Menu<'_, Message, Theme, Renderer> {
        Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0)
    }
    fn submenu_button<'a>(label: &str) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
        Self::base(
            row![
                text(label)
                    .width(Length::Fill)
                    .vertical_alignment(alignment::Vertical::Center),
                text(BootstrapIcon::CaretRightFill)
                    .font(BOOTSTRAP_FONT)
                    .width(Length::Shrink)
                    .vertical_alignment(alignment::Vertical::Center),
            ]
            .align_items(iced::Alignment::Center),
            Message::Tick(Instant::now()),
        )
        .width(Length::Fill)
    }

    fn menu<'a>() -> Menu<'a, Message, Theme, Renderer>;

    fn item<'a>(self) -> Item<'a, Message, Theme, Renderer>;
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    Rotate(bool),
    CloseAlert,
    Preset(PresetMessage),
    Conway(ConwayMessage),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug, Clone, EnumIter)]
pub enum PresetMessage {
    Prism(usize),
    AntiPrism(usize),
    Pyramid(usize),
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl std::fmt::Display for PresetMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PresetMessage::*;
        match self {
            Prism(n) => match n {
                3 => f.write_str("Triangular"),
                4 => f.write_str("Cube"),
                5 => f.write_str("Pentagonal"),
                6 => f.write_str("Hexagonal"),
                7 => f.write_str("Heptagonal"),
                8 => f.write_str("Octagonal"),
                _ => f.write_str("?"),
            },
            Pyramid(n) => match n {
                3 => f.write_str("Tetrahedron"),
                4 => f.write_str("Square"),
                5 => f.write_str("Pentagonal"),
                6 => f.write_str("Hexagonal"),
                7 => f.write_str("Heptagonal"),
                8 => f.write_str("Octagonal"),
                _ => f.write_str("?"),
            },
            _ => f.write_fmt(format_args!("{self:?}")),
        }
    }
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ConwayMessage {
    // 1
    Dual,
    // 2
    // Join,
    Ambo,
    // 3
    // Kis,
    // Needle,
    // Zip,
    Truncate,
    // 4
    // Ortho,
    Expand,
    // 5
    // Gyro,
    // Snub,
    // // 6
    // Meta,
    Bevel,
    Contract,
}

impl MenuAble for PresetMessage {
    fn item<'a>(self) -> Item<'a, Message, Theme, Renderer> {
        Item::new(Self::labeled(&self.to_string(), Message::Preset(self)))
    }

    fn menu<'a>() -> Menu<'a, Message, Theme, Renderer> {
        let items: Vec<Item<'a, Message, Theme, Renderer>> = PresetMessage::iter()
            .map(|message| {
                use PresetMessage::*;
                match message {
                    Prism(_) => Item::with_menu(
                        Self::submenu_button("Prism"),
                        Self::base_menu((3..=8).map(Prism).map(Self::item).collect()),
                    ),
                    AntiPrism(_) => Item::with_menu(
                        Self::submenu_button("AntiPrism"),
                        Self::base_menu((3..=8).map(AntiPrism).map(Self::item).collect()),
                    ),
                    Pyramid(_) => Item::with_menu(
                        Self::submenu_button("Pyramid"),
                        Self::base_menu((3..=8).map(Pyramid).map(Self::item).collect()),
                    ),
                    _ => Self::item(message),
                }
            })
            .collect();
        Self::base_menu(items)
    }
}

impl MenuAble for ConwayMessage {
    fn item<'a>(self) -> Item<'a, Message, Theme, Renderer> {
        Item::new(Self::labeled(&self.to_string(), Message::Conway(self)).width(Length::Fill))
    }

    fn menu<'a>() -> Menu<'a, Message, Theme, Renderer> {
        let items: Vec<Item<'a, Message, Theme, Renderer>> =
            ConwayMessage::iter().map(Self::item).collect();
        Self::base_menu(items)
    }
}
