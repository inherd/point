use crate::print_ui::editor::Editor;
use crate::print_ui::navigation_bar::NavigationBar;
use crate::print_ui::project_tool_window::ProjectToolWindow;
use crate::print_ui::status_bar::StatusBar;
use iced::{
    executor, keyboard, Align, Application, Clipboard, Color, Column, Command, Container, Element,
    Length, Row, Settings, Subscription,
};
use iced_native::{event, subscription, Event};

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
        (Self::default(), Command::none())
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
                }) if modifiers.is_command_pressed() => handle_hotkey(key_code),
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {
        let top =
            Row::with_children(vec![NavigationBar::render("hello.dat").into()]).width(Length::Fill);

        let middle = Row::with_children(vec![
            ProjectToolWindow::render().into(),
            Editor::render().into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill);

        let bottom = Row::with_children(vec![StatusBar::render().into()])
            .width(Length::Fill)
            .align_items(iced::Align::End);

        let row = Column::with_children(vec![top.into(), middle.into(), bottom.into()])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Start);

        Container::new(row).style(style::MainView).into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb(242.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0)
    }
}

fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<Message> {
    match key_code {
        _ => Some(Message::BackPressed),
    }
}

mod style {
    use iced::{container, Color};

    pub struct MainView;

    impl container::StyleSheet for MainView {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::BLACK),
                ..container::Style::default()
            }
        }
    }
}
