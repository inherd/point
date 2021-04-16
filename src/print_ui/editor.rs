pub struct Editor {
    pub input_value: String,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            input_value: "".to_string(),
        }
    }
}

impl Editor {
    pub fn render(&mut self) {}
}
