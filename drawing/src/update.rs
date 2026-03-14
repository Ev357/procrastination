use crate::{
    message::{Message, mouse::MouseMessage},
    model::{Model, mode::Mode, mouse_state::MouseState, running_state::RunningState},
};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running_state = RunningState::Done;

            None
        }
        Message::Mouse(mouse_message) => match mouse_message {
            MouseMessage::Down(column, row) => {
                model.mouse_state = MouseState::Pressed;

                Some(Message::UpdatePixel(column, row))
            }
            MouseMessage::Drag(column, row) => {
                if let MouseState::Pressed = model.mouse_state {
                    return Some(Message::UpdatePixel(column, row));
                }

                None
            }
            MouseMessage::Up => {
                model.history.push(model.pixels.clone());
                model.mouse_state = MouseState::Released;

                None
            }
        },
        Message::UpdatePixel(column, row) => match model.mode {
            Mode::Draw => {
                model.pixels.insert((column, row));

                None
            }
            Mode::Erase => {
                model.pixels.remove(&(column, row));

                None
            }
        },
        Message::SwitchMode => {
            model.mode = match model.mode {
                Mode::Draw => Mode::Erase,
                Mode::Erase => Mode::Draw,
            };

            None
        }
        Message::Clear => {
            model.pixels.clear();
            model.history.push(model.pixels.clone());

            None
        }
        Message::Undo => {
            if let Some(pixels) = model.history.undo() {
                model.pixels = pixels;
            }

            None
        }
        Message::Redo => {
            if let Some(pixels) = model.history.redo() {
                model.pixels = pixels;
            }

            None
        }
    }
}
