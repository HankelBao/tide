extern crate syntect;

use crate::terminal::DisplayLine;

pub struct TextBuffer {
    pub line_num: u32,
    pub line_offset: u32,

    pub top_line: u32,
    pub left_col: u32,

    pub lines: Vec<Box<String>>,
    pub rendered_cache: Vec<DisplayLine>,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        let textbuffer = TextBuffer {
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            lines: vec![Box::new(String::new())],
            rendered_cache: Vec::new(),
        };
        return textbuffer;
    }

}