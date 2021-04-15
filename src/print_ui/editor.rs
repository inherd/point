use crate::Message;
use iced::{Container, Element, Text};

pub struct Editor {}

impl Editor {
    pub fn render<'a>() -> Element<'a, Message> {
        let editor = Text::new("Editor").size(50);

        Container::new(editor).into()
    }
}
