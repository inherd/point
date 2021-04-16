use crate::message::Message;
use crate::print_ui::ToolWindow;
use iced::{Container, Element, HorizontalAlignment, Length, Text, VerticalAlignment};

pub struct ProjectToolWindow {}

impl ToolWindow for ProjectToolWindow {}

impl ProjectToolWindow {
    pub fn render<'a>() -> Element<'a, Message> {
        let element = Text::new("Project TW")
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top);

        Container::new(element)
            .style(style::ProjectToolWindow)
            .height(Length::Fill)
            .into()
    }
}

mod style {
    use iced::{container, Background, Color};

    pub struct ProjectToolWindow;

    impl container::StyleSheet for ProjectToolWindow {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color([1.0, 1.0, 1.0].into())),
                text_color: Some(Color::BLACK),
                ..container::Style::default()
            }
        }
    }
}
