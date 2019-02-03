extern crate syntect;

use crate::terminal::DisplayLine;
use super::SyntaxReference;

pub struct TextBuffer<'a> {
    pub line_num: u32,
    pub line_offset: u32,

    pub top_line: u32,
    pub left_col: u32,

    pub lines: Vec<Box<String>>,
    pub rendered_cache: Vec<DisplayLine>,

    pub file_path: String,
    pub syntax: &'a SyntaxReference,
    pub syntax_name: String,
}

impl<'a> TextBuffer<'a> {
    pub fn new(default_syntax_ref: &'a SyntaxReference) -> TextBuffer<'a> {
        let textbuffer = TextBuffer {
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            lines: vec![Box::new(String::new())],
            rendered_cache: Vec::new(),
            file_path: String::new(),
            syntax: default_syntax_ref,
            syntax_name: default_syntax_ref.name.clone(),
        };
        textbuffer
    }

    pub fn as_string(&self) -> String {
        let mut content_string: String = String::new();
        for line in &self.lines {
            content_string.push_str(line);
            content_string.push('\n');
        }
        content_string
    }
}