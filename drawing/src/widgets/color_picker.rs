use std::collections::HashMap;

use crossterm::style::Color;

use crate::{
    model::pixel::Pixel,
    widgets::{Widget, area::Area},
};

const TITLE: &str = " Color Picker ";
const PALETTE: [Color; 16] = [
    Color::White,
    Color::Grey,
    Color::DarkGrey,
    Color::Black,
    Color::Red,
    Color::DarkRed,
    Color::Yellow,
    Color::DarkYellow,
    Color::Green,
    Color::DarkGreen,
    Color::Cyan,
    Color::DarkCyan,
    Color::Blue,
    Color::DarkBlue,
    Color::Magenta,
    Color::DarkMagenta,
];
const SWATCH_WIDTH: u16 = 4;
const MAX_SWATCHES_PER_ROW: usize = 5;

#[derive(Debug, Default, Clone)]
pub struct ColorPicker {
    pub focused_index: usize,
}

impl ColorPicker {
    pub fn right(&mut self) {
        self.focused_index = (self.focused_index + 1) % PALETTE.len();
    }

    pub fn left(&mut self) {
        self.focused_index = (self.focused_index + PALETTE.len() - 1) % PALETTE.len();
    }

    pub fn down(&mut self) {
        if self.focused_index + MAX_SWATCHES_PER_ROW < PALETTE.len() {
            self.focused_index += MAX_SWATCHES_PER_ROW;
        }
    }

    pub fn up(&mut self) {
        if self.focused_index >= MAX_SWATCHES_PER_ROW {
            self.focused_index -= MAX_SWATCHES_PER_ROW;
        }
    }

    pub fn current_color(&self) -> Color {
        PALETTE[self.focused_index]
    }
}

impl Widget for ColorPicker {
    fn render(&self, area: &Area, buffer: &mut HashMap<(u16, u16), Pixel>) {
        let end_column = area.x + area.width.saturating_sub(1);
        let end_row = area.y + area.height.saturating_sub(1);

        for column in area.x..=end_column {
            for row in area.y..=end_row {
                let character = match (column, row) {
                    (c, r) if c == area.x && r == area.y => '╭',
                    (c, r) if c == end_column && r == area.y => '╮',
                    (c, r) if c == area.x && r == end_row => '╰',
                    (c, r) if c == end_column && r == end_row => '╯',
                    (_, r) if r == area.y || r == end_row => '─',
                    (c, _) if c == area.x || c == end_column => '│',
                    _ => ' ',
                };

                buffer.insert(
                    (column, row),
                    Pixel {
                        color: Color::White,
                        character,
                    },
                );
            }
        }

        let title_len = TITLE.chars().count() as u16;
        let title_start = area.x + (area.width.saturating_sub(title_len) / 2);

        for (index, character) in TITLE.chars().enumerate() {
            let column = title_start + index as u16;

            if column < end_column {
                buffer.insert(
                    (column, area.y),
                    Pixel {
                        color: Color::White,
                        character,
                    },
                );
            }
        }

        let inner_start_x = area.x + 2;
        let inner_start_y = area.y + 2;
        let inner_width = area.width.saturating_sub(SWATCH_WIDTH);

        let max_swatches_per_row = (inner_width / SWATCH_WIDTH).max(1);

        for (index, &color) in PALETTE.iter().enumerate() {
            let row_offset = (index as u16) / max_swatches_per_row;
            let col_offset = (index as u16) % max_swatches_per_row;

            let current_x = inner_start_x + (col_offset * SWATCH_WIDTH);
            let current_y = inner_start_y + (row_offset * 2);

            if current_y >= end_row {
                break;
            }

            let left_char = if index == self.focused_index {
                '['
            } else {
                ' '
            };
            let right_char = if index == self.focused_index {
                ']'
            } else {
                ' '
            };

            buffer.insert(
                (current_x, current_y),
                Pixel {
                    color: Color::White,
                    character: left_char,
                },
            );

            buffer.insert(
                (current_x + 1, current_y),
                Pixel {
                    color,
                    character: '█',
                },
            );

            buffer.insert(
                (current_x + 2, current_y),
                Pixel {
                    color,
                    character: '█',
                },
            );

            buffer.insert(
                (current_x + 3, current_y),
                Pixel {
                    color: Color::White,
                    character: right_char,
                },
            );
        }
    }
}
