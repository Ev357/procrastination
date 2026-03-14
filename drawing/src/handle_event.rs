use color_eyre::eyre::Result;
use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};

use crate::{message::Message, message::mouse::MouseMessage, model::Model};

pub fn handle_event(_: &Model) -> Result<Option<Message>> {
    #[allow(clippy::single_match)]
    let messsage = match event::read()? {
        Event::Key(event) => match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Char('q') => Some(Message::Quit),
                KeyCode::Char('s') => Some(Message::SwitchMode),
                KeyCode::Char('c') => Some(Message::Clear),
                KeyCode::Char('u') => Some(Message::Undo),
                KeyCode::Char('r') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Message::Redo)
                }
                _ => None,
            },
            _ => None,
        },
        Event::Mouse(event) => match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                Some(Message::Mouse(MouseMessage::Down(event.column, event.row)))
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                Some(Message::Mouse(MouseMessage::Drag(event.column, event.row)))
            }
            MouseEventKind::Up(MouseButton::Left) => Some(Message::Mouse(MouseMessage::Up)),
            _ => None,
        },
        _ => None,
    };

    Ok(messsage)
}
