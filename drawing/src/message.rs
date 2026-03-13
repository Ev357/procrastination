#[derive(Debug)]
pub enum Message {
    Quit,
    Mouse(u16, u16),
    SwitchMode,
    Clear,
}
