use color_eyre::eyre::Result;

use crate::{
    handle_event::handle_event, running_state::RunningState, terminal::Terminal, update::update,
};

mod handle_event;
mod message;
mod mode;
mod model;
mod running_state;
mod terminal;
mod update;

fn main() -> Result<()> {
    color_eyre::install()?;

    run()
}

fn run() -> Result<()> {
    let mut terminal = Terminal::default();
    terminal.init()?;

    while let RunningState::Running = terminal.model.running_state {
        let mut current_msg = handle_event(&terminal.model)?;

        while let Some(message) = current_msg {
            current_msg = terminal.update(|model| update(model, message))?;
        }
    }

    Ok(())
}
