use std::sync::{Arc, Mutex, mpsc};
use super::TextLine;

pub type MessageKey = u32;

pub enum Message<T> {
    Other(T),
    CursorMove(usize, (u16, u16)),
    HighlightReady(usize),
    StartBufferHighlight(usize, Arc<Mutex<Vec<TextLine>>>, mpsc::Sender<(u32, u32)>),
}
