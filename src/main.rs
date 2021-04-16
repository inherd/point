use iced::{
    executor, keyboard, pane_grid, scrollable, Application, Clipboard, Color, Column, Command,
    Container, Element, Length, PaneGrid, Row, Rule, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};

use message::Message;

use crate::print_ui::editor::Editor;
use crate::print_ui::navigation_bar::NavigationBar;
use crate::print_ui::status_bar::StatusBar;
use crate::print_ui::tool_window::ProjectToolWindow;

pub mod message;
pub mod print_ui;

pub fn main() -> iced::Result {
    PrintUI::run(Settings::default())
}

struct PrintUI {
    panes: pane_grid::State<Content>,
}

struct Content {
    id: usize,
    scroll: scrollable::State,
}

impl Content {
    fn new(id: usize) -> Self {
        Content {
            id,
            scroll: scrollable::State::new(),
        }
    }
}

impl Application for PrintUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (panes, _) = pane_grid::State::new(Content::new(0));

        (PrintUI { panes }, Command::none())
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

        let pane_grid = PaneGrid::new(&mut self.panes, |pane, content| {
            let title = ProjectToolWindow::render();
            let title_bar = pane_grid::TitleBar::new(title).padding(10);

            pane_grid::Content::new(ProjectToolWindow::render()).title_bar(title_bar)
        });

        let middle = Row::with_children(vec![
            ProjectToolWindow::render().into(),
            pane_grid.into(),
            PrintUI::vertical_rule(),
            Editor::render().into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill);

        let bottom = Row::with_children(vec![StatusBar::render().into()]).width(Length::Fill);

        let row = Column::with_children(vec![
            PrintUI::horizontal_rule().into(),
            top.into(),
            PrintUI::horizontal_rule().into(),
            middle.into(),
            PrintUI::horizontal_rule().into(),
            bottom.into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill);

        Container::new(row).style(style::MainView).into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb(242.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0)
    }
}

impl PrintUI {
    fn horizontal_rule<'a>() -> Element<'a, Message> {
        Rule::horizontal(1).style(style::Rule).into()
    }

    fn vertical_rule<'a>() -> Element<'a, Message> {
        Rule::vertical(1).style(style::Rule).into()
    }
}

fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<Message> {
    match key_code {
        _ => Some(Message::BackPressed),
    }
}

mod style {
    use iced::{container, rule, Color};

    pub struct MainView;

    impl container::StyleSheet for MainView {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::BLACK),
                ..container::Style::default()
            }
        }
    }

    pub struct Rule;

    const SURFACE: Color = Color::from_rgb(
        0x40 as f32 / 255.0,
        0x44 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: SURFACE,
                width: 1,
                radius: 1.0,
                fill_mode: rule::FillMode::Full,
            }
        }
    }
}
