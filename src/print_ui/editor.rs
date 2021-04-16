use crate::message::Message;
use iced::{Container, Element, HorizontalAlignment, Text, VerticalAlignment};

pub struct Editor {}

impl Editor {
    pub fn render<'a>() -> Element<'a, Message> {
        let editor = Text::new("Editor")
            .size(50)
            .horizontal_alignment(HorizontalAlignment::Right)
            .vertical_alignment(VerticalAlignment::Top);

        Container::new(editor).into()
    }
}
