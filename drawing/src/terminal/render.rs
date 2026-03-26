use color_eyre::eyre::Result;
use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};
use std::{collections::HashMap, io::Write};

use crate::{
    model::{Model, pixel::Pixel},
    terminal::Terminal,
};

impl Terminal {
    pub fn render(&mut self, model: &Model) -> Result<()> {
        let mut old_screen: HashMap<(u16, u16), Pixel> = HashMap::new();
        for layer in &self.model.pixels {
            for (position, pixel) in layer {
                old_screen.insert(*position, pixel.clone());
            }
        }

        let mut new_screen: HashMap<(u16, u16), Pixel> = HashMap::new();
        for layer in &model.pixels {
            for (position, pixel) in layer {
                new_screen.insert(*position, pixel.clone());
            }
        }

        for (position @ (column, row), pixel) in &new_screen {
            if old_screen.get(position) == Some(pixel) {
                continue;
            }

            self.stdout
                .queue(cursor::MoveTo(*column, *row))?
                .queue(style::PrintStyledContent(pixel.character.with(pixel.color)))?;
        }

        for position @ (column, row) in old_screen.keys() {
            if new_screen.contains_key(position) {
                continue;
            }

            self.stdout
                .queue(cursor::MoveTo(*column, *row))?
                .queue(style::Print(' '))?;
        }

        self.stdout.flush()?;

        Ok(())
    }
}
