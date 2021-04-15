use crate::Message;
use iced::{Container, Element, HorizontalAlignment, Text, VerticalAlignment};

pub struct NavigationBar {}

impl NavigationBar {
    pub fn render<'a>(path: &str) -> Element<'a, Message> {
        let element = Text::new(path)
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top);

        Container::new(element).into()
    }
}

mod style {}
