use std::io::Write;

use color_eyre::eyre::Result;
use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};

use crate::{model::Model, terminal::Terminal};

impl Terminal {
    pub fn render(&mut self, model: &Model) -> Result<()> {
        let added = model.pixels.difference(&self.model.pixels);

        for (column, row) in added {
            self.stdout
                .queue(cursor::MoveTo(*column, *row))?
                .queue(style::PrintStyledContent('█'.white()))?;
        }

        let removed = self.model.pixels.difference(&model.pixels);
        for (column, row) in removed {
            self.stdout
                .queue(cursor::MoveTo(*column, *row))?
                .queue(style::Print(' '))?;
        }

        self.stdout.flush()?;

        Ok(())
    }
}
