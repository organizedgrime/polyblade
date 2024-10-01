use super::{controls::Controls, message::RenderMessage, state::RenderState};
use crate::render::message::{ColorMethodMessage, ConwayMessage, PolybladeMessage, PresetMessage};
use iced::{
    alignment::{self, Vertical},
    overlay::menu::Catalog,
    theme,
    widget::{button, checkbox, row, slider, text, Button},
    Border, Length, Renderer,
};
use iced_aw::{
    menu::{Item, Menu},
    menu_bar,
    // Bootstrap, BOOTSTRAP_FONT,
};
use iced_winit::runtime::Program;
use std::{fmt::Display, ops::RangeInclusive};
use strum::IntoEnumIterator;

pub trait MenuAble<'a, C: Program>: Display + Clone + Sized
where
    C::Message: 'a + Clone,
    C::Theme:
        'a + iced_aw::menu::Catalog + iced_widget::button::Catalog + iced_widget::text::Catalog,
    C::Renderer: 'a,
{
    type State;
    const TITLE: &'static str;

    fn transform(message: Self) -> C::Message;
    fn menu_items(state: &Self::State) -> Vec<Item<'a, C::Message, C::Theme, C::Renderer>>;

    /*
    fn menu<'a>(state: &Self::State) -> Menu<'a, C::Message, Theme, Renderer> {
        Self::new_menu(Self::menu_items(state))
    }

    fn new_menu(
        items: Vec<Item<'_, C::Message, Theme, Renderer>>,
    ) -> Menu<'_, PolybladeMessage, Theme, Renderer> {
        Menu::new(items).max_width(180.0).offset(10.0).spacing(5.0)
    }
    */

    fn title() -> Button<'a, C::Message, C::Theme, C::Renderer> {
        button(row![
            text(Self::TITLE).align_y(Vertical::Center) // text(Bootstrap::CaretDownFill)
                                                        //     .size(18)
                                                        //     .font(BOOTSTRAP_FONT)
                                                        //     .height(Length::Shrink)
        ])
        .width(Length::Shrink)
        // .style(theme::Button::custom(LotusButton))
    }

    fn button(self) -> Item<'a, C::Message, C::Theme, C::Renderer> {
        Item::new(
            button(text(self.to_string()).align_y(Vertical::Center))
                .padding([4, 8])
                .on_press(Self::transform(self))
                // .style(theme::Button::custom(LotusButton))
                .width(Length::Fill),
        )
    }
    /*

    fn checkbox<'a, F>(
        label: &str,
        checked: bool,
        on_toggle: F,
    ) -> Item<'a, C::Message, Theme, Renderer>
    where
        F: 'a + Fn(bool) -> Self,
    {
        Item::new(
            checkbox(label, checked)
                .on_toggle(move |v| Self::transform(on_toggle(v)))
                .width(Length::Fill),
        )
    }

    fn slider<'a, F>(
        range: RangeInclusive<f32>,
        value: f32,
        on_slide: F,
        step: f32,
    ) -> Item<'a, C::Message, Theme, Renderer>
    where
        F: 'a + Fn(f32) -> Self,
    {
        Item::new(slider(range, value, move |v| Self::transform(on_slide(v))).step(step))
    }

    fn submenu<'a>(
        label: &'a str,
        items: Vec<Self>,
    ) -> Item<'a, C::Message, iced::Theme, iced::Renderer> {
        Item::with_menu(
            button(
                row![
                    text(label).width(Length::Fill).align_y(Vertical::Center) // text(Bootstrap::CaretRightFill)
                                                                              //     .font(BOOTSTRAP_FONT)
                                                                              //     .width(Length::Shrink)
                                                                              //     .vertical_alignment(alignment::Vertical::Center),
                ], // .align_x(Horizontal::Center),
            )
            .padding([4, 8])
            // .style(theme::Button::custom(LotusButton))
            .width(Length::Fill),
            Self::new_menu(items.into_iter().map(Self::button).collect()),
        )
    }
    */
}

impl MenuAble<'static, Controls> for PresetMessage {
    type State = ();
    const TITLE: &'static str = "Preset";

    fn transform(message: Self) -> <Controls as Program>::Message {
        <Controls as Program>::Message::Preset(message)
    }

    fn menu_items(
        _: &(),
    ) -> Vec<
        Item<
            'static,
            <Controls as Program>::Message,
            <Controls as Program>::Theme,
            <Controls as Program>::Renderer,
        >,
    > {
        use PresetMessage::*;
        vec![
            // Self::submenu("Prism", (3..=8).map(Prism).collect()),
            // Self::submenu("AntiPrism", (3..=8).map(AntiPrism).collect()),
            // Self::submenu("Pyramid", (3..=8).map(Pyramid).collect()),
            Self::button(Octahedron),
            Self::button(Dodecahedron),
            Self::button(Icosahedron),
        ]
    }
}

/*
impl MenuAble for ConwayMessage {
    type State = ();
    const TITLE: &'static str = "Conway";

    fn transform(message: Self) -> PolybladeMessage {
        PolybladeMessage::Conway(message)
    }

    fn menu_items<'a>(_: &()) -> Vec<Item<'a, PolybladeMessage, Theme, Renderer>> {
        ConwayMessage::iter().map(Self::button).collect()
    }
}

impl MenuAble for RenderMessage {
    type State = RenderState;
    const TITLE: &'static str = "Render";

    fn transform(message: Self) -> PolybladeMessage {
        PolybladeMessage::Render(message)
    }

    fn menu_items<'a>(state: &Self::State) -> Vec<Item<'a, PolybladeMessage, Theme, Renderer>> {
        use RenderMessage::*;
        vec![
            Self::checkbox("Schlegel", state.schlegel, Schlegel),
            Self::checkbox("Rotating", state.rotating, Rotating),
            Self::slider(0.0..=10.0, state.line_thickness, LineThickness, 1.0),
            Self::slider(
                0.0..=(std::f32::consts::PI * 2.0),
                state.camera.fov_y,
                FovChanged,
                0.1,
            ),
            Self::submenu(
                "Color Method",
                ColorMethodMessage::iter().map(ColorMethod).collect(),
            ),
        ]
    }
}

*/
/* impl PolybladeMessage {
    pub fn menu_bar(&self, render: &RenderState) -> Item<'_, Self, Theme, Renderer> {
        let menu_bar = menu_bar!((PresetMessage::title(), PresetMessage::menu(&()))(
            ConwayMessage::title(),
            ConwayMessage::menu(&())
        )(
            RenderMessage::title(), RenderMessage::menu(render)
        ));
        menu_bar.into()
    }
} */
/*
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
*/

pub struct ColorPickerBox {
    pub color: iced::Color,
}

/*
impl button::StyleSheet for ColorPickerBox {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.color)),
            ..Default::default()
        }
    }
}
*/
