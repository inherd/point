use crate::message::Message;
use iced::{
    text_input, Container, Element, HorizontalAlignment, Length, Text, TextInput, VerticalAlignment,
};
use iced_native::Widget;

pub struct Editor {
    input: text_input::State,
    pub input_value: String,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            input: Default::default(),
            input_value: "".to_string(),
        }
    }
}

impl Editor {
    pub fn render(&mut self) -> Element<Message> {
        let editor = TextInput::new(
            &mut self.input,
            "What needs to be done?",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(14)
        .width(Length::Fill);

        Container::new(editor)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
