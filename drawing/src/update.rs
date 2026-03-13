use crate::{message::Message, mode::Mode, model::Model, running_state::RunningState};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::Mouse(column, row) => match model.mode {
            Mode::Draw => {
                model.pixels.insert((column, row));
            }
            Mode::Erase => {
                model.pixels.remove(&(column, row));
            }
        },
        Message::SwitchMode => {
            model.mode = match model.mode {
                Mode::Draw => Mode::Erase,
                Mode::Erase => Mode::Draw,
            };
        }
        Message::Clear => {
            model.pixels.clear();
        }
    };

    None
}
