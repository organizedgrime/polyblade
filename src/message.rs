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

#[derive(Debug, Clone, EnumIter, Display)]
pub enum PresetMessage {
    Prism(usize),
    Pyramid(usize),
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
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
                    Prism(_) => Item::new(Text::new("nyaaaa")),
                    Pyramid(_) => Item::with_menu(
                        Self::submenu_button("Pyramid"),
                        Self::base_menu(vec![Item::new(Text::new("three"))]),
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
