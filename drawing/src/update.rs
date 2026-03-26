use crossterm::terminal;

use crate::{
    message::{Message, direction::Direction, mouse::MouseMessage},
    model::{
        Model, mode::Mode, mouse_state::MouseState, pixel::Pixel, running_state::RunningState,
        screen_state::ScreenState,
    },
    widgets::{Widget, area::Area},
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
                model.history.push(model.pixels[0].clone());
                model.mouse_state = MouseState::Released;

                None
            }
        },
        Message::UpdatePixel(column, row) => {
            if let ScreenState::ColorPicker = model.screen_state {
                return None;
            }

            match model.mode {
                Mode::Draw => {
                    model.pixels[0].insert(
                        (column, row),
                        Pixel {
                            color: model.color_picker.current_color(),
                            character: model.character,
                        },
                    );

                    None
                }
                Mode::Erase => {
                    model.pixels[0].remove(&(column, row));

                    None
                }
            }
        }
        Message::SwitchMode => {
            model.mode = match model.mode {
                Mode::Draw => Mode::Erase,
                Mode::Erase => Mode::Draw,
            };

            None
        }
        Message::Clear => {
            model.pixels[0].clear();
            model.history.push(model.pixels[0].clone());

            None
        }
        Message::Undo => {
            if let Some(pixels) = model.history.undo() {
                model.pixels[0] = pixels;
            }

            None
        }
        Message::Redo => {
            if let Some(pixels) = model.history.redo() {
                model.pixels[0] = pixels;
            }

            None
        }
        Message::ColorPicker => {
            if let ScreenState::Draw = model.screen_state {
                model.screen_state = ScreenState::ColorPicker;

                let (width, height) = match terminal::size() {
                    Ok(size) => size,
                    Err(error) => return Some(Message::Error(error.into())),
                };

                let area = Area {
                    width: 24,
                    height: 12,
                    x: (width / 2) - 12,
                    y: (height / 2) - 6,
                };

                model.color_picker.render(&area, &mut model.pixels[1]);
            } else {
                model.screen_state = ScreenState::Draw;
                model.pixels[1].clear();
            }

            None
        }
        Message::Confirm | Message::Close => {
            if let ScreenState::ColorPicker = model.screen_state {
                model.screen_state = ScreenState::Draw;
                model.pixels[1].clear();
            }

            None
        }
        Message::Direction(direction) => {
            if let ScreenState::Draw = model.screen_state {
                return None;
            }

            match direction {
                Direction::Right => {
                    model.color_picker.right();
                }
                Direction::Left => {
                    model.color_picker.left();
                }
                Direction::Up => {
                    model.color_picker.up();
                }
                Direction::Down => {
                    model.color_picker.down();
                }
            };

            let (width, height) = match terminal::size() {
                Ok(size) => size,
                Err(error) => return Some(Message::Error(error.into())),
            };

            let area = Area {
                width: 24,
                height: 12,
                x: (width / 2) - 12,
                y: (height / 2) - 6,
            };

            model.pixels[1].clear();
            model.color_picker.render(&area, &mut model.pixels[1]);

            None
        }
        Message::Error(error) => {
            panic!("{}", error);
        }
    }
}
