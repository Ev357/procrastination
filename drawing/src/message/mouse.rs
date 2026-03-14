#[derive(Debug)]
pub enum MouseMessage {
    Down(u16, u16),
    Drag(u16, u16),
    Up,
}
