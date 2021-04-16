use crate::message::Message;
use iced::{Container, Element, HorizontalAlignment, Text, VerticalAlignment};

pub struct StatusBar {}
impl StatusBar {
    pub fn render<'a>() -> Element<'a, Message> {
        let element = Text::new("status")
            .horizontal_alignment(HorizontalAlignment::Right)
            .vertical_alignment(VerticalAlignment::Bottom);

        Container::new(element).into()
    }
}
