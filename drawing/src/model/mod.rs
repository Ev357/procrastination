use std::collections::HashMap;

use crate::{
    model::{
        history::History, mode::Mode, mouse_state::MouseState, pixel::Pixel,
        running_state::RunningState, screen_state::ScreenState,
    },
    widgets::color_picker::ColorPicker,
};

pub mod history;
pub mod mode;
pub mod mouse_state;
pub mod pixel;
pub mod running_state;
pub mod screen_state;

#[derive(Debug, Clone)]
pub struct Model {
    pub pixels: [HashMap<(u16, u16), Pixel>; 2],
    pub running_state: RunningState,
    pub mouse_state: MouseState,
    pub mode: Mode,
    pub history: History,
    pub screen_state: ScreenState,
    pub character: char,
    pub color_picker: ColorPicker,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            pixels: [HashMap::new(), HashMap::new()],
            running_state: RunningState::default(),
            mouse_state: MouseState::default(),
            mode: Mode::default(),
            history: History::default(),
            screen_state: ScreenState::default(),
            character: '█',
            color_picker: ColorPicker::default(),
        }
    }
}
