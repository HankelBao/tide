extern crate syntect;

use super::TextLine;

pub struct TextBuffer {
    pub line_num: u32,
    pub line_offset: u32,

    pub top_line: u32,
    pub left_col: u32,

    pub lines: Vec<Box<TextLine>>,

    pub file_path: String,
    pub syntax_name: String,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        let textbuffer = TextBuffer {
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            lines: vec![Box::new(TextLine::new())],
            file_path: String::new(),
            syntax_name: String::new(),
        };
        textbuffer
    }

    pub fn as_string(&self) -> String {
        let mut content_string: String = String::new();
        for line in self.lines.iter() {
            content_string.push_str(&line.content());
            content_string.push('\n');
        }
        content_string
    }
}