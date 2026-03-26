use crossterm::style::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pixel {
    pub color: Color,
    pub character: char,
}
