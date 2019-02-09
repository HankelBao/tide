use std::sync::{Arc, Mutex, mpsc};
use super::TextLine;
use crate::terminal::Key;

pub type MessageSender = mpsc::Sender<Message>;

#[derive(Clone)]
pub enum Message {
    TerminalKey(Key),
    FocusFileUpdate(String),
    FocusCursorMove(u16, u16),
    HighlightReady(usize),
    Quit,
}
