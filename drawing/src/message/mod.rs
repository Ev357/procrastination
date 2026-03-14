use crate::message::mouse::MouseMessage;

pub mod mouse;

#[derive(Debug)]
pub enum Message {
    Quit,
    SwitchMode,
    Clear,
    Mouse(MouseMessage),
    UpdatePixel(u16, u16),
    Undo,
    Redo,
}
