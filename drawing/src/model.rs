use std::collections::HashSet;

use crate::{mode::Mode, running_state::RunningState};

#[derive(Debug, Default, Clone)]
pub struct Model {
    pub pixels: HashSet<(u16, u16)>,
    pub running_state: RunningState,
    pub mode: Mode,
}
