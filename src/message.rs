use std::time::Instant;

use iced::{
    alignment, font, theme,
    widget::{button, row, text},
    Border, Color, Element, Length, Renderer, Theme,
};
use iced_aw::{
    menu::{Item, Menu},
    BootstrapIcon, BOOTSTRAP_FONT,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub fn bar<'a>(label: &str) -> button::Button<'a, Message, Theme, Renderer> {
    button(row![
        text(label).vertical_alignment(alignment::Vertical::Center),
        text(BootstrapIcon::CaretDownFill)
            .size(18)
            .font(BOOTSTRAP_FONT)
            .height(Length::Shrink)
    ])
    .width(Length::Shrink)
    .style(theme::Button::custom(LotusButton))
}
pub fn base<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, Theme, Renderer> {
    button(content)
        .padding([4, 8])
        .on_press(msg)
        .style(theme::Button::custom(LotusButton))
}
fn labeled<'a>(label: &str, msg: Message) -> button::Button<'a, Message, Theme, Renderer> {
    base(
        text(label).vertical_alignment(alignment::Vertical::Center),
        msg,
    )
    .width(Length::Fill)
}

fn base_menu(items: Vec<Item<'_, Message, Theme, Renderer>>) -> Menu<'_, Message, Theme, Renderer> {
    Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0)
}
fn submenu_button<'a>(label: &str) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    base(
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

pub trait MenuAble {
    fn menu<'a>() -> Menu<'a, Message, Theme, Renderer>;

    fn item<'a>(self) -> Item<'a, Message, Theme, Renderer>;
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    Rotate(bool),
    SizeChanged(f32),
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
            AntiPrism(n) => match n {
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
    Join,
    Ambo,
    // 3
    Kis,
    // Needle,
    // Zip,
    Truncate,
    // 4
    //Ortho,
    Expand,
    // 5
    // Gyro,
    Snub,
    // // 6
    // Meta,
    Bevel,
    Contract,
}

impl MenuAble for PresetMessage {
    fn item<'a>(self) -> Item<'a, Message, Theme, Renderer> {
        Item::new(labeled(&self.to_string(), Message::Preset(self)))
    }

    fn menu<'a>() -> Menu<'a, Message, Theme, Renderer> {
        let items: Vec<Item<'a, Message, Theme, Renderer>> = PresetMessage::iter()
            .map(|message| {
                use PresetMessage::*;
                match message {
                    Prism(_) => Item::with_menu(
                        submenu_button("Prism"),
                        base_menu((3..=8).map(Prism).map(Self::item).collect()),
                    ),
                    AntiPrism(_) => Item::with_menu(
                        submenu_button("AntiPrism"),
                        base_menu((3..=8).map(AntiPrism).map(Self::item).collect()),
                    ),
                    Pyramid(_) => Item::with_menu(
                        submenu_button("Pyramid"),
                        base_menu((3..=8).map(Pyramid).map(Self::item).collect()),
                    ),
                    _ => Self::item(message),
                }
            })
            .collect();
        base_menu(items)
    }
}

impl MenuAble for ConwayMessage {
    fn item<'a>(self) -> Item<'a, Message, Theme, Renderer> {
        Item::new(labeled(&self.to_string(), Message::Conway(self)).width(Length::Fill))
    }

    fn menu<'a>() -> Menu<'a, Message, Theme, Renderer> {
        let items: Vec<Item<'a, Message, Theme, Renderer>> =
            ConwayMessage::iter().map(Self::item).collect();
        base_menu(items)
    }
}

struct LotusButton;
impl button::StyleSheet for LotusButton {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let palette = Theme::KanagawaLotus.extended_palette();

        button::Appearance {
            background: Some(palette.secondary.base.color.into()),
            text_color: palette.secondary.base.text,
            border: Border::with_radius(5),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let palette = Theme::KanagawaLotus.extended_palette();
        button::Appearance {
            background: Some(palette.primary.base.color.into()),
            ..self.active(&Theme::KanagawaLotus)
        }
    }
}
