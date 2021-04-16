#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    InputChanged(String),
}
