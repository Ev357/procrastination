use std::collections::{HashMap, VecDeque};

use crate::model::pixel::Pixel;

#[derive(Debug, Clone)]
pub struct History {
    pub history: VecDeque<HashMap<(u16, u16), Pixel>>,
    pub position: u8,
}

impl History {
    pub fn push(&mut self, pixels: HashMap<(u16, u16), Pixel>) {
        self.history.truncate(self.position as usize + 1);

        if self.history.len() > 9 {
            self.history.pop_front();
        }

        self.history.push_back(pixels);
        self.position = self.history.len() as u8 - 1;
    }

    pub fn undo(&mut self) -> Option<HashMap<(u16, u16), Pixel>> {
        if self.position == 0 {
            return None;
        }

        self.position -= 1;

        self.history.get(self.position as usize).cloned()
    }

    pub fn redo(&mut self) -> Option<HashMap<(u16, u16), Pixel>> {
        if self.position == self.history.len() as u8 - 1 {
            return None;
        }

        self.position += 1;

        self.history.get(self.position as usize).cloned()
    }
}

impl Default for History {
    fn default() -> Self {
        Self {
            history: VecDeque::from([HashMap::new()]),
            position: 0,
        }
    }
}
