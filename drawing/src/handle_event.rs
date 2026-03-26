use color_eyre::eyre::Result;
use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};

use crate::{
    message::{Message, direction::Direction, mouse::MouseMessage},
    model::Model,
};

pub fn handle_event(_: &Model) -> Result<Option<Message>> {
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
                KeyCode::Char('h') | KeyCode::Left => Some(Message::Direction(Direction::Left)),
                KeyCode::Char('l') | KeyCode::Right => Some(Message::Direction(Direction::Right)),
                KeyCode::Char('k') | KeyCode::Up => Some(Message::Direction(Direction::Up)),
                KeyCode::Char('j') | KeyCode::Down => Some(Message::Direction(Direction::Down)),
                KeyCode::Enter => Some(Message::Confirm),
                KeyCode::Esc => Some(Message::Close),
                KeyCode::Char('p') => Some(Message::ColorPicker),
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
