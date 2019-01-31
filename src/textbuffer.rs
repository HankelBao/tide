pub struct CursorPos {
    line_num: u32,
    line_offset: u32,
}

impl CursorPos {
    fn from(line_offset: u32, line_num: u32) -> CursorPos {
        return CursorPos {
            line_num: line_num,
            line_offset: line_offset,
        };
    }

    fn to(&mut self, x: u32, y: u32) {
        self.line_num = y;
        self.line_offset = x;
    }

    fn left(&mut self, left_bound: u32) {
        if self.line_offset <= left_bound {
            return;
        }
        self.line_offset -= 1;
    }

    fn right(&mut self, right_bound: u32) {
        if self.line_offset >= right_bound {
            return;
        }
        self.line_offset += 1;
    }

    fn up(&mut self, up_bound: u32) {
        if self.line_offset <= up_bound {
            return;
        }
        self.line_num -= 1;
    }

    fn down(&mut self, down_bound: u32) {
        if self.line_offset >= down_bound {
            return;
        }
        self.line_num += 1;
    }
}

pub struct TextBuffer {
    cursor_pos: CursorPos,
    lines: Vec<Box<String>>,
    rendered_cached: Vec<Option<String>>,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        let textbuffer = TextBuffer {
            cursor_pos: CursorPos::from(0, 0),
            lines: vec![Box::new(String::new())],
            rendered_cached: vec![None],
        };
        return textbuffer;
    }

    pub fn insert_at(&mut self, line_offset: u32, line_num: u32, ch: char) {
        self.lines[line_offset as usize].insert(line_offset as usize + 1, ch);
    }

    pub fn backspace_at(&mut self, line_offset: u32, line_num: u32) {
        if line_offset > 0 {

        }
    }
}