use iced::tooltip::{self, Tooltip};
use iced::{button, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Sandbox, Settings, Text, VerticalAlignment, window};

pub mod print_ui;

pub fn main() {
    Example::run(Settings::default()).unwrap()
}

#[derive(Default)]
struct Example {
    top: button::State,
    bottom: button::State,
    right: button::State,
    left: button::State,
}

#[derive(Debug, Clone, Copy)]
struct Message;

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Tooltip - Iced")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&mut self) -> Element<Message> {
        let top = Button::new(
            &mut self.bottom,
            Text::new("Bottom")
                .size(40)
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
            .on_press(Message)
            .width(Length::Fill)
            .height(Length::Fill)
            // .style(style::Button)
            ;

        let fixed_tooltips = Row::with_children(vec![
            top.into(),
        ])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(50);

        let content = Column::with_children(vec![
            Container::new(fixed_tooltips)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into(),
        ])
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(50);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(50)
            .into()
    }
}

mod style {
    use iced::container;
    use iced::Color;

    pub struct Button;

    impl container::StyleSheet for Button {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::from_rgb8(0xEE, 0xEE, 0xEE)),
                background: Some(Color::from_rgb(0.11, 0.42, 0.87).into()),
                border_radius: 12.0,
                ..container::Style::default()
            }
        }
    }
}
