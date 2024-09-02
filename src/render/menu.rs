use std::{fmt::Display, ops::RangeInclusive};

use iced::{
    alignment, theme,
    widget::{button, checkbox, row, slider, text},
    Border, Element, Length, Renderer, Theme,
};
use iced_aw::{
    menu::{Item, Menu},
    menu_items, Bootstrap, BOOTSTRAP_FONT,
};
use strum::IntoEnumIterator;

use crate::{
    render::message::{ConwayMessage, PolybladeMessage, PresetMessage},
    Instant,
};

use super::{
    message::RenderMessage,
    state::{AppState, RenderState},
};

pub trait MenuAble: Display + Clone + Sized {
    type State;

    fn transform(message: Self) -> PolybladeMessage;
    fn menu_items<'a>(state: &Self::State) -> Vec<Item<'a, PolybladeMessage, Theme, Renderer>>;

    fn menu<'a>(state: &Self::State) -> Menu<'a, PolybladeMessage, Theme, Renderer> {
        Self::new_menu(Self::menu_items(state))
    }

    fn new_menu<'a>(
        items: Vec<Item<'a, PolybladeMessage, Theme, Renderer>>,
    ) -> Menu<'a, PolybladeMessage, Theme, Renderer> {
        Menu::new(items).max_width(180.0).offset(10.0).spacing(5.0)
    }

    fn item_button<'a>(self) -> Item<'a, PolybladeMessage, Theme, Renderer> {
        Item::new(Self::labeled_button_with_message(&self.to_string(), self))
    }

    fn button<'a>(label: &str) -> button::Button<'a, PolybladeMessage, Theme, Renderer> {
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

    fn button_with_message<'a>(
        content: impl Into<Element<'a, PolybladeMessage, Theme, Renderer>>,
        msg: Self,
    ) -> button::Button<'a, PolybladeMessage, Theme, Renderer> {
        button(content)
            .padding([4, 8])
            .on_press(Self::transform(msg))
            .style(theme::Button::custom(LotusButton))
    }

    fn labeled_button_with_message<'a>(
        label: &str,
        msg: Self,
    ) -> button::Button<'a, PolybladeMessage, Theme, Renderer> {
        Self::button_with_message(
            text(label).vertical_alignment(alignment::Vertical::Center),
            msg,
        )
        .width(Length::Fill)
    }

    fn labeled_checkbox_with_message<'a, F>(
        label: &str,
        checked: bool,
        on_toggle: F,
    ) -> checkbox::Checkbox<'a, PolybladeMessage, Theme, Renderer>
    where
        F: 'a + Fn(bool) -> Self,
    {
        checkbox(label, checked)
            .on_toggle(move |v| Self::transform(on_toggle(v)))
            .width(Length::Fill)
    }

    fn item_checkbox<'a, F>(
        label: &str,
        checked: bool,
        on_toggle: F,
    ) -> Item<'a, PolybladeMessage, Theme, Renderer>
    where
        F: 'a + Fn(bool) -> Self,
    {
        Item::new(Self::labeled_checkbox_with_message(
            &label, checked, on_toggle,
        ))
    }

    fn item_slider<'a, F>(
        range: RangeInclusive<f32>,
        value: f32,
        on_slide: F,
    ) -> Item<'a, PolybladeMessage, Theme, Renderer>
    where
        F: 'a + Fn(f32) -> Self,
    {
        Item::new(slider(range, value, move |v| Self::transform(on_slide(v))))
    }

    fn values_to_menu<'a>(items: Vec<Self>) -> Menu<'a, PolybladeMessage, Theme, Renderer> {
        Self::new_menu(items.into_iter().map(Self::item_button).collect())
    }

    fn submenu_button<'a>(
        label: &str,
    ) -> button::Button<'a, PolybladeMessage, iced::Theme, iced::Renderer> {
        button(
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
        )
        .padding([4, 8])
        .style(theme::Button::custom(LotusButton))
        .width(Length::Fill)
    }
}

impl MenuAble for PresetMessage {
    type State = ();

    fn transform(message: Self) -> PolybladeMessage {
        PolybladeMessage::Preset(message)
    }

    fn menu_items<'a>(_: &()) -> Vec<Item<'a, PolybladeMessage, Theme, Renderer>> {
        PresetMessage::iter()
            .map(|message| {
                use PresetMessage::*;
                match message {
                    Prism(_) => Item::with_menu(
                        Self::submenu_button("Prism"),
                        Self::values_to_menu((3..=8).map(Prism).collect()),
                    ),
                    AntiPrism(_) => Item::with_menu(
                        Self::submenu_button("AntiPrism"),
                        Self::values_to_menu((3..=8).map(AntiPrism).collect()),
                    ),
                    Pyramid(_) => Item::with_menu(
                        Self::submenu_button("Pyramid"),
                        Self::values_to_menu((3..=8).map(Pyramid).collect()),
                    ),
                    _ => Self::item_button(message),
                }
            })
            .collect()
    }
}

impl MenuAble for ConwayMessage {
    type State = ();

    fn transform(message: Self) -> PolybladeMessage {
        PolybladeMessage::Conway(message)
    }

    fn menu_items<'a>(_: &()) -> Vec<Item<'a, PolybladeMessage, Theme, Renderer>> {
        ConwayMessage::iter().map(Self::item_button).collect()
    }
}

impl MenuAble for RenderMessage {
    type State = RenderState;

    fn transform(message: Self) -> PolybladeMessage {
        PolybladeMessage::Render(message)
    }

    fn menu_items<'a>(state: &Self::State) -> Vec<Item<'a, PolybladeMessage, Theme, Renderer>> {
        vec![
            Self::item_checkbox("Schlegel", state.schlegel, RenderMessage::Schlegel),
            Self::item_checkbox("Rotating", state.rotating, RenderMessage::Rotating),
            Self::item_slider(
                0.0..=10.0,
                state.line_thickness,
                RenderMessage::LineThickness,
            ),
        ]
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
