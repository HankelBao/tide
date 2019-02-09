extern crate syntect;

use super::TextLine;
use super::HighlightEngine;
use super::SyntaxHighlight;
use super::FileRW;
use std::sync::{Arc, Mutex, mpsc};
use crate::core::Message;

pub struct TextBuffer {
    pub buffer_index: usize,
    pub line_num: u32,
    pub line_offset: u32,

    pub top_line: u32,
    pub left_col: u32,
    pub view_height: u32,

    pub lines: Arc<Mutex<Vec<Box<TextLine>>>>,
    pub messagesender: mpsc::Sender<Message>,

    pub highlight_send: Option<mpsc::Sender<(u32, u32)>>,

    pub file_path: String,
    pub syntax_name: String,
}

impl TextBuffer {
    pub fn new(buffer_index: usize, messagesender: mpsc::Sender<Message>) -> TextBuffer {
        let mut textbuffer = TextBuffer {
            buffer_index,
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            view_height: 0,
            lines: Arc::new(Mutex::new(vec![Box::new(TextLine::new())])),
            messagesender,
            highlight_send: None,
            file_path: String::new(),
            syntax_name: String::new(),
        };
        textbuffer.highlight_from(0);
        textbuffer
    }

    pub fn from_file(buffer_index: usize, messagesender: mpsc::Sender<Message>, file_path: String) -> TextBuffer {
        let mut textbuffer = TextBuffer {
            buffer_index,
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            view_height: 100,
            lines: Arc::new(Mutex::new(vec![Box::new(TextLine::new())])),
            messagesender,
            highlight_send: None,
            file_path: String::new(),
            syntax_name: String::new(),
        };
        textbuffer.set_file_path(file_path);
        textbuffer.load_file();
        textbuffer.highlight_from(0);
        textbuffer
    }

    pub fn as_string(&self) -> String {
        let mut content_string: String = String::new();
        let lines = self.lines.lock().unwrap();
        for line in lines.iter() {
            content_string.push_str(&line.content());
            content_string.push('\n');
        }
        content_string
    }
}
