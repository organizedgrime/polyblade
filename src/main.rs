mod message;
mod polyhedra;
mod scene;

use iced_aw::{
    card,
    menu::{Item, Menu},
    menu_bar, menu_items, modal, BootstrapIcon, BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES,
};
use message::*;
use polyhedra::Transaction;
use scene::Scene;

use iced::time::Instant;
use iced::widget::shader::wgpu;
use iced::widget::{button, checkbox, column, container, row, shader, text, Row, Text};
use iced::{alignment, window};
use iced::{executor, Border, Color};
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
use strum::IntoEnumIterator;

fn main() -> iced::Result {
    Polyblade::run(iced::Settings::default())
}

struct Polyblade {
    start: Instant,
    scene: Scene,
    rotating: bool,
    show_alert: bool,
}

impl Application for Polyblade {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Polyblade")
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                start: Instant::now(),
                scene: Scene::new(),
                rotating: true,
                show_alert: false,
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;
        match message {
            Tick(time) => {
                self.scene.update(time - self.start);
            }
            Rotate(rotating) => {
                self.rotating = rotating;
            }
            CloseAlert => {
                self.show_alert = false;
            }
            Preset(preset) => self.scene.polyhedron.change_shape(preset),
            Conway(conway) => {
                self.scene
                    .polyhedron
                    .transactions
                    .push(Transaction::Conway(conway));
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        let menu_tpl_2 = |items| Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0);
        let meow = menu_bar!((debug_button_s("Nested Menus"), {
            let sub1 = menu_tpl_2(menu_items!((debug_button("Item"))(debug_button("Item"))(
                debug_button("Item")
            )(debug_button("Item"))(
                debug_button("Item")
            )))
            .width(220.0);

            menu_tpl_1(menu_items!((debug_button("Item"))(debug_button("Item"))(
                submenu_button("A sub menu"),
                sub1
            )(debug_button("Item"))(
                debug_button("Item")
            )(debug_button("Item"))))
            .width(140.0)
        }));

        //let conway_buttons =             .collect();
        let preset_row = Row::with_children(PresetMessage::iter().map(|message| {
            button(Text::new(message.to_string()))
                .on_press(Message::Preset(message))
                .into()
        }))
        .spacing(10);
        let conway_row = Row::with_children(ConwayMessage::iter().map(|message| {
            button(Text::new(message.to_string()))
                .on_press(Message::Conway(message))
                .into()
        }))
        .spacing(10);

        let underlay = container(
            column![
                meow,
                shader(&self.scene).width(Length::Fill).height(Length::Fill),
                checkbox("Rotating", self.rotating).on_toggle(Message::Rotate),
                preset_row,
                conway_row,
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(10);

        let card = if self.show_alert {
            Some(
                card("Error", "Sorry, that isn't implemented yet.")
                    .foot(
                        row![button("Ok")
                            .width(Length::Fill)
                            .on_press(Message::CloseAlert)]
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill),
                    )
                    .max_width(300.0)
                    .on_close(Message::CloseAlert),
            )
        } else {
            None
        };

        modal(underlay, card)
            .backdrop(Message::CloseAlert)
            .on_esc(Message::CloseAlert)
            //.align_y(Vertical::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        use iced::keyboard;
        use keyboard::key;
        let handle_hotkey = |key: key::Key, _modifiers: keyboard::Modifiers| match key.as_ref() {
            keyboard::Key::Character("d") => Some(Message::Conway(ConwayMessage::Dual)),
            keyboard::Key::Character("e") => Some(Message::Conway(ConwayMessage::Expand)),
            keyboard::Key::Character("c") => Some(Message::Conway(ConwayMessage::Contract)),
            keyboard::Key::Character("a") => Some(Message::Conway(ConwayMessage::Ambo)),
            keyboard::Key::Character("t") => Some(Message::Conway(ConwayMessage::Truncate)),
            keyboard::Key::Character("b") => Some(Message::Conway(ConwayMessage::Bevel)),
            _ => None,
        };
        let tick = window::frames().map(Message::Tick);
        Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
    }
}

struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            background: Some(Color::TRANSPARENT.into()),
            // background: Some(Color::from([1.0; 3]).into()),
            border: Border {
                radius: [6.0; 4].into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Theme, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}
fn labeled_button<'a>(
    label: &str,
    msg: Message,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    base_button(
        text(label).vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}
fn debug_button<'a>(label: &str) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Tick(Instant::now())).width(Length::Fill)
}
fn debug_button_s<'a>(label: &str) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Tick(Instant::now())).width(Length::Shrink)
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
