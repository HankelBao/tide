extern crate syntect;

use super::TextLine;

use super::HighlightEngine;
use std::sync::{Arc, Mutex, mpsc};


pub struct TextBuffer {
    pub line_num: u32,
    pub line_offset: u32,

    pub top_line: u32,
    pub left_col: u32,

    pub lines: Arc<Mutex<Vec<Box<TextLine>>>>,

    pub file_path: String,
    pub syntax_name: String,
}

impl TextBuffer {
    pub fn new<'a>(highlightengine: &'a HighlightEngine) -> TextBuffer {
        let textbuffer = TextBuffer {
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            lines: Arc::new(Mutex::new(vec![Box::new(TextLine::new())])),
            file_path: String::new(),
            syntax_name: String::new(),
        };
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