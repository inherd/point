use iced::tooltip::{self, Tooltip};
use iced::{Column, Container, Element, Length, Row, Sandbox, Settings, Text};

pub mod print_ui;

pub fn main() -> iced::Result {
    PrintUI::run(Settings::default())
}

#[derive(Default)]
struct PrintUI {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    BackPressed,
    NextPressed,
}

impl Sandbox for PrintUI {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Print - Editor")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {}
            Message::NextPressed => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new().spacing(20).push(Text::new("Hello, World").size(50));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(50)
            .into()

    }
}

