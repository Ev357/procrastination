use std::io;

use color_eyre::eyre::Result;
use crossterm::{ExecutableCommand, cursor, event, terminal};

use crate::model::Model;

pub mod render;
pub mod update;

#[derive(Debug)]
pub struct Terminal {
    pub model: Model,
    pub stdout: io::Stdout,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if let Err(report) = self.restore() {
            println!("Error while restoring terminal: {}", report);
        }
    }
}

impl Terminal {
    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        self.stdout.execute(event::EnableMouseCapture)?;
        self.stdout.execute(cursor::Hide)?;

        Ok(())
    }

    fn restore(&mut self) -> Result<()> {
        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(event::DisableMouseCapture)?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            model: Model::default(),
            stdout: io::stdout(),
        }
    }
}
