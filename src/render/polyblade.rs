use iced::mouse;
use iced::widget::{button, Row};
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
    window, Application, Command, Element, Length, Subscription, Theme,
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
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(PolybladeMessage::FontLoaded),
                font::load(iced_aw::NERD_FONT_BYTES).map(PolybladeMessage::FontLoaded),
                Command::perform(async { load_polydex() }, |polydex| {
                    PolybladeMessage::PolydexLoaded(polydex.map_err(|err| err.to_string()))
                }),
            ]),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        message.process(&mut self.state)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut button_row = Row::new().spacing(10);

        for (i, color) in self.state.render.picker.palette.colors.iter().enumerate() {
            button_row = button_row.push(
                button("")
                    .style(iced::theme::Button::Custom(Box::new(ColorPickerBox {
                        color: (*color).into(),
                    })))
                    .width(20)
                    .height(20)
                    .on_press(PolybladeMessage::Render(RenderMessage::ColorPicker(
                        ColorPickerMessage::ChooseColor(i),
                    ))),
            );
        }

        let cp = color_picker(
            self.state.render.picker.color_index.is_some(),
            self.state.render.picker.picked_color,
            text("").width(0).height(0),
            PolybladeMessage::Render(RenderMessage::ColorPicker(ColorPickerMessage::CancelColor)),
            |v| {
                PolybladeMessage::Render(RenderMessage::ColorPicker(
                    ColorPickerMessage::SubmitColor(v),
                ))
            },
        );

        container(
            column![
                row![menu_bar!((
                    PresetMessage::title(),
                    PresetMessage::menu(&())
                )(
                    ConwayMessage::title(),
                    ConwayMessage::menu(&())
                )(
                    RenderMessage::title(),
                    RenderMessage::menu(&self.state.render)
                ))]
                .spacing(10.0),
                button_row,
                // Actual shader of the program
                container(shader(self).width(Length::Fill).height(Length::Fill)),
                // Info
                column![
                    container(column![
                        button(text(self.state.info.name()))
                            .on_press(self.state.info.wiki_message()),
                        row![
                            column![
                                text("Bowers:"),
                                text("Conway:"),
                                text("Faces:"),
                                text("Edges:"),
                                text("Vertices:"),
                            ],
                            column![
                                text(self.state.info.bowers()),
                                text(&self.state.info.conway),
                                text(self.state.info.faces),
                                text(self.state.info.edges),
                                text(self.state.info.vertices),
                            ]
                        ]
                        .spacing(20)
                    ]),
                    row![
                        text("Colors: "),
                        text(self.state.render.picker.colors.to_string()),
                        slider(
                            1..=self.state.render.picker.palette.colors.len() as i16,
                            self.state.render.picker.colors,
                            |x| PolybladeMessage::Render(RenderMessage::ColorPicker(
                                ColorPickerMessage::ChangeNumber(x)
                            ))
                        )
                        .step(1i16)
                    ],
                    row![
                        text("Size: "),
                        text(self.state.model.scale.to_string()),
                        slider(0.0..=10.0, self.state.model.scale, |v| {
                            PolybladeMessage::Model(ModelMessage::ScaleChanged(v))
                        })
                        .step(0.1)
                    ],
                ]
            ]
            .spacing(10)
            .push(cp),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(10)
        .into()
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

impl<Message> shader::Program<Message> for Polyblade {
    type State = ();
    type Primitive = PolyhedronPrimitive;

    /* fn update(
        &self,
        _state: &mut Self::State,
        event: shader::Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
        _shell: &mut Shell<'_, Message>,
    ) -> (event::Status, Option<Message>) {
        match event {
            /* shader::Event::Mouse(_) => {}
            shader::Event::Touch(_) => {}
            shader::Event::Keyboard(_) => {} */
            shader::Event::RedrawRequested(time) => (event::Status::Captured, None),
            _ => (event::Status::Ignored, None),
        }
    } */

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        Self::Primitive::new(self.state.model.clone(), self.state.render.clone())
    }
}
