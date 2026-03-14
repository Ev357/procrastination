use std::collections::HashSet;

use crate::model::{
    history::History, mode::Mode, mouse_state::MouseState, running_state::RunningState,
};

pub mod history;
pub mod mode;
pub mod mouse_state;
pub mod running_state;

#[derive(Debug, Default, Clone)]
pub struct Model {
    pub pixels: HashSet<(u16, u16)>,
    pub running_state: RunningState,
    pub mouse_state: MouseState,
    pub mode: Mode,
    pub history: History,
}
