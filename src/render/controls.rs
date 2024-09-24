use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use iced_aw::{color_picker, menu::Item, menu_bar};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container, row, shader, slider, text, Row};
use iced_winit::core::{Color, Element, Length::*, Theme};
use iced_winit::runtime::{Program, Task};

use crate::render::{
    menu::{ColorPickerBox, MenuAble as _},
    message::*,
    state::AppState,
};

pub struct Controls {
    pub state: AppState,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    pub fn background_color(&self) -> Color {
        self.state.render.background_color
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Theme = Theme;
    type Message = PolybladeMessage;

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        message.process(&mut self.state)
    }

    fn view(&self) -> Element<Self::Message, Self::Theme, Self::Renderer> {
        let mut button_row = Row::new().spacing(10);

        for (i, color) in self.state.render.picker.palette.colors.iter().enumerate() {
            button_row = button_row.push(
                button("")
                    // .style(Button::Custom(Box::new(ColorPickerBox {
                    //     color: (*color).into(),
                    // })))
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
                // container(shader(self.state).width(Length::Fill).height(Length::Fill)),
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
        // .width(Length::Fill)
        // .height(Length::Fill)
        // .align_x(Horizontal::Center)
        // .align_y(Vertical::Center)
        .padding(10);

        container(column![text("Background color").color(Color::WHITE),].spacing(10))
            .padding(10)
            .align_bottom(Fill)
            .into()
    }
}
