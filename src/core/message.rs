use std::sync::mpsc;
use crate::terminal::Key;

pub type MessageSender = mpsc::Sender<Message>;

#[derive(Clone)]
pub enum Message {
    TerminalKey(Key),
    TerminalResize(u16, u16),
    RedrawAllUIComponents,
    FocusFileUpdate(String),
    FocusCursorMove(u16, u16),
    FocusSyntaxUpdate(String),
    HighlightReady(usize),
    Quit,
}
