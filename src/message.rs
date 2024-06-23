use std::time::Instant;

use iced::{
    alignment,
    widget::{button, row, text, Row, Text},
    Element, Length, Renderer, Theme,
};
use iced_aw::{
    menu::{Item, Menu},
    menu_bar, menu_items, BootstrapIcon, BOOTSTRAP_FONT,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::base_button;

pub trait MenuAble {
    fn bar<'a>(label: &str) -> button::Button<'a, Message, Theme, Renderer> {
        button(text(label).vertical_alignment(alignment::Vertical::Center))
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

    fn base_menu<'a>(
        items: Vec<Item<'a, Message, Theme, Renderer>>,
    ) -> Menu<'a, Message, Theme, Renderer> {
        Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0)
    }
    fn submenu_button<'a>(label: &str) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
        base_button(
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
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    Rotate(bool),
    CloseAlert,
    Preset(PresetMessage),
    Conway(ConwayMessage),
}

#[derive(Debug, Clone, EnumIter)]
pub enum PresetMessage {
    Prism(usize),
    Pyramid(usize),
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl std::fmt::Display for PresetMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PresetMessage::*;
        match self {
            Prism(n) => match n {
                3 => f.write_str("Pen"),
                _ => f.write_str("meow"),
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
    /*
    fn view() -> Element<'static, Message> {
        Row::with_children(PresetMessage::iter().map(|message| {
            use PresetMessage::*;
            match message {
                Prism(_) => Self::labeled("Prism", Message::Tick(Instant::now())).into(),
                Pyramid(_) => Self::labeled("Pyramid", Message::Tick(Instant::now())).into(),
                _ => button(Text::new(message.to_string()))
                    .on_press(Message::Preset(message))
                    .into(),
            }
        }))
        .spacing(10)
        .into()
    }
    */
    fn menu<'a>() -> Menu<'a, Message, Theme, Renderer> {
        let items: Vec<Item<'a, Message, Theme, Renderer>> = PresetMessage::iter()
            .map(|message| {
                use PresetMessage::*;
                match message {
                    Prism(_) => Item::with_menu(
                        Self::submenu_button("Prism"),
                        Self::base_menu(
                            (3..=8)
                                .into_iter()
                                .map(|n| Prism(n))
                                .map(|msg| {
                                    Item::new(Self::labeled(&msg.to_string(), Message::Preset(msg)))
                                })
                                .collect(),
                        ),
                    ),
                    Pyramid(_) => Item::with_menu(
                        Self::submenu_button("Pyramid"),
                        Self::base_menu(
                            (3..=8)
                                .into_iter()
                                .map(|n| Pyramid(n))
                                .map(|msg| {
                                    Item::new(Self::labeled(&msg.to_string(), Message::Preset(msg)))
                                })
                                .collect(),
                        ),
                    ),
                    _ => Item::new(
                        Self::base(Text::new(message.to_string()), Message::Preset(message))
                            .width(Length::Fill),
                    ),
                }
            })
            .collect();
        Self::base_menu(items)
    }
}

/*
impl ViewAble for ConwayMessage {
    fn view() -> Element<'static, Message> {
        Row::with_children(ConwayMessage::iter().map(|message| {
            button(Text::new(message.to_string()))
                .on_press(Message::Conway(message))
                .into()
        }))
        .spacing(10)
        .into()
    }
}

impl ViewAble for Message {
    fn view() -> impl Into<Element<'static, Message>> {
        menu_bar!((button("Preset"), Self::menu(vec![])))
    }
}
*/
