use iced::{Application, Clipboard, Column, Command, Container, Element, executor, keyboard, Length, Row, Settings, Subscription, Text, Color};
use iced_native::{event, Event, subscription};

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

impl Application for PrintUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Print - Editor")
    }

    fn update(&mut self, event: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match event {
            Message::BackPressed => {}
            Message::NextPressed => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    modifiers,
                                    key_code,
                                }) if modifiers.is_command_pressed() => { handle_hotkey(key_code) }
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new().spacing(20).push(Text::new("Hello, World").size(50));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(50)
            .style(style::Container)
            .into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb(
            242.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0,
        )
    }
}

fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<Message> {
    match key_code {
        _ => Some(Message::BackPressed)
    }
}

mod style {
    use iced::{container, Color};

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::BLACK),
                ..container::Style::default()
            }
        }
    }
}