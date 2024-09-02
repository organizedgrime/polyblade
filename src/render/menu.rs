use iced::{
    alignment, theme,
    widget::{button, checkbox, row, slider, text},
    Border, Element, Length, Renderer, Theme,
};
use iced_aw::{
    menu::{Item, Menu},
    Bootstrap, BOOTSTRAP_FONT,
};
use strum::IntoEnumIterator;

use crate::{
    render::message::{ConwayMessage, PolybladeMessage, PresetMessage},
    Instant,
};

use super::{message::RenderMessage, state::AppState};

pub fn bar<'a>(label: &str) -> button::Button<'a, PolybladeMessage, Theme, Renderer> {
    button(row![
        text(label).vertical_alignment(alignment::Vertical::Center),
        text(Bootstrap::CaretDownFill)
            .size(18)
            .font(BOOTSTRAP_FONT)
            .height(Length::Shrink)
    ])
    .width(Length::Shrink)
    .style(theme::Button::custom(LotusButton))
}

pub fn base<'a>(
    content: impl Into<Element<'a, PolybladeMessage, Theme, Renderer>>,
    msg: PolybladeMessage,
) -> button::Button<'a, PolybladeMessage, Theme, Renderer> {
    button(content)
        .padding([4, 8])
        .on_press(msg)
        .style(theme::Button::custom(LotusButton))
}

fn labeled<'a>(
    label: &str,
    msg: PolybladeMessage,
) -> button::Button<'a, PolybladeMessage, Theme, Renderer> {
    base(
        text(label).vertical_alignment(alignment::Vertical::Center),
        msg,
    )
    .width(Length::Fill)
}

fn base_menu(
    items: Vec<Item<'_, PolybladeMessage, Theme, Renderer>>,
) -> Menu<'_, PolybladeMessage, Theme, Renderer> {
    Menu::new(items).max_width(180.0).offset(10.0).spacing(5.0)
}

fn submenu_button<'a>(
    label: &str,
) -> button::Button<'a, PolybladeMessage, iced::Theme, iced::Renderer> {
    base(
        row![
            text(label)
                .width(Length::Fill)
                .vertical_alignment(alignment::Vertical::Center),
            text(Bootstrap::CaretRightFill)
                .font(BOOTSTRAP_FONT)
                .width(Length::Shrink)
                .vertical_alignment(alignment::Vertical::Center),
        ]
        .align_items(iced::Alignment::Center),
        PolybladeMessage::Tick(Instant::now()),
    )
    .width(Length::Fill)
}

pub trait MenuAble {
    fn menu<'a>(state: &AppState) -> Menu<'a, PolybladeMessage, Theme, Renderer>;

    fn item<'a>(self) -> Item<'a, PolybladeMessage, Theme, Renderer>;
}

impl MenuAble for PresetMessage {
    fn item<'a>(self) -> Item<'a, PolybladeMessage, Theme, Renderer> {
        Item::new(labeled(&self.to_string(), PolybladeMessage::Preset(self)))
    }

    fn menu<'a>(_state: &AppState) -> Menu<'a, PolybladeMessage, Theme, Renderer> {
        let items: Vec<Item<'a, PolybladeMessage, Theme, Renderer>> = PresetMessage::iter()
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
    fn item<'a>(self) -> Item<'a, PolybladeMessage, Theme, Renderer> {
        Item::new(labeled(&self.to_string(), PolybladeMessage::Conway(self)).width(Length::Fill))
    }

    fn menu<'a>(_state: &AppState) -> Menu<'a, PolybladeMessage, Theme, Renderer> {
        let items: Vec<Item<'a, PolybladeMessage, Theme, Renderer>> =
            ConwayMessage::iter().map(Self::item).collect();
        base_menu(items)
    }
}

impl MenuAble for RenderMessage {
    fn item<'a>(self) -> Item<'a, PolybladeMessage, Theme, Renderer> {
        Item::new(labeled(&self.to_string(), PolybladeMessage::Render(self)).width(Length::Fill))
    }

    fn menu<'a>(state: &AppState) -> Menu<'a, PolybladeMessage, Theme, Renderer> {
        base_menu(vec![
            Item::new(
                checkbox(
                    RenderMessage::Schlegel(false).to_string(),
                    state.render.schlegel,
                )
                .on_toggle(|v| PolybladeMessage::Render(RenderMessage::Schlegel(v))),
            ),
            Item::new(
                checkbox(
                    RenderMessage::Rotating(false).to_string(),
                    state.render.rotating,
                )
                .on_toggle(|v| PolybladeMessage::Render(RenderMessage::Rotating(v))),
            ),
            Item::new(
                slider(1.0..=10.0, state.render.line_thickness, |v| {
                    PolybladeMessage::Render(RenderMessage::LineThickness(v))
                })
                .step(0.1),
            ),
        ])
    }
}

struct LotusButton;
impl button::StyleSheet for LotusButton {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let palette = Theme::Light.extended_palette();

        button::Appearance {
            background: Some(palette.secondary.base.color.into()),
            text_color: palette.secondary.base.text,
            border: Border::with_radius(5),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let palette = Theme::Light.extended_palette();
        button::Appearance {
            background: Some(palette.primary.base.color.into()),
            ..self.active(&Theme::Light)
        }
    }
}

pub struct ColorPickerBox {
    pub color: iced::Color,
}

impl button::StyleSheet for ColorPickerBox {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.color)),
            ..Default::default()
        }
    }
}
