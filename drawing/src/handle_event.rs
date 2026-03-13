use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};

use crate::{message::Message, model::Model};

pub fn handle_event(_: &Model) -> Result<Option<Message>> {
    #[allow(clippy::single_match)]
    let messsage = match event::read()? {
        Event::Key(event) => match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Char('q') => Some(Message::Quit),
                KeyCode::Char('s') => Some(Message::SwitchMode),
                KeyCode::Char('c') => Some(Message::Clear),
                _ => None,
            },
            _ => None,
        },
        Event::Mouse(event) => match event.kind {
            MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Down(MouseButton::Left) => {
                Some(Message::Mouse(event.column, event.row))
            }
            _ => None,
        },
        _ => None,
    };

    Ok(messsage)
}
