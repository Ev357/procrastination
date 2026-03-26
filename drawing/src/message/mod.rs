use color_eyre::eyre::Error;

use crate::message::{direction::Direction, mouse::MouseMessage};

pub mod direction;
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
    ColorPicker,
    Error(Error),
    Direction(Direction),
    Confirm,
    Close,
}
