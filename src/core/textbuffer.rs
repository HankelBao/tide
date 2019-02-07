extern crate syntect;

use super::TextLine;
use super::HighlightEngine;
use super::SyntaxHighlight;
use super::FileRW;
use std::sync::{Arc, Mutex, mpsc};

pub struct TextBuffer {
    pub line_num: u32,
    pub line_offset: u32,

    pub top_line: u32,
    pub left_col: u32,
    pub view_height: u32,

    pub lines: Arc<Mutex<Vec<Box<TextLine>>>>,
    pub highlight_send: Option<mpsc::Sender<(u32, u32)>>,
    pub display_send: Arc<Mutex<mpsc::Sender<bool>>>,
    pub display_recv: Arc<Mutex<mpsc::Receiver<bool>>>,

    pub file_path: String,
    pub syntax_name: String,
}

impl TextBuffer {
    pub fn new(highlightengine: &HighlightEngine) -> TextBuffer {
        let (send, recv) = mpsc::channel();
        let mut textbuffer = TextBuffer {
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            view_height: 0,
            lines: Arc::new(Mutex::new(vec![Box::new(TextLine::new())])),
            file_path: String::new(),
            syntax_name: String::new(),
            display_send: Arc::new(Mutex::new(send.clone())),
            display_recv: Arc::new(Mutex::new(recv)),
            highlight_send: None,
        };
        send.clone().send(true).unwrap();
        textbuffer.start_highlight_thread(highlightengine);
        textbuffer.highlight_from(0);
        textbuffer
    }

    pub fn from_file(highlightengine: &HighlightEngine, file_path: String) -> TextBuffer {
        let (send, recv) = mpsc::channel();
        let mut textbuffer = TextBuffer {
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            view_height: 100,
            lines: Arc::new(Mutex::new(vec![Box::new(TextLine::new())])),
            file_path: String::new(),
            syntax_name: String::new(),
            display_send: Arc::new(Mutex::new(send.clone())),
            display_recv: Arc::new(Mutex::new(recv)),
            highlight_send: None,
        };
        textbuffer.set_file_path(file_path);
        textbuffer.load_file();
        send.clone().send(true).unwrap();
        textbuffer.start_highlight_thread(highlightengine);
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