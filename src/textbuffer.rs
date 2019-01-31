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

    fn set(&mut self, x: u32, y: u32) {
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

struct LineInfo {
    buffer_index: usize,
    buffer_offset: usize,
    length: usize,
    rendered_string: Option<String>,
}

impl LineInfo {
    fn new() -> LineInfo {
        return LineInfo {
            buffer_index: 0,
            buffer_offset: 0,
            length: 0,
            rendered_string: None,
        }
    }
}

pub struct TextBuffer {
    cursor_pos: CursorPos,
    buffers: Vec<Box<String>>,
    line_info: Vec<Box<LineInfo>>,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        let mut textbuffer = TextBuffer {
            cursor_pos: CursorPos::from(0, 0),
            buffers: vec![Box::new(String::from(""))],
            line_info: vec![Box::new(LineInfo::new())],
        };
        return textbuffer;
    }

    pub fn insert_at(&mut self, line_offset: u32, line_num: u32, ch: char) {
        let buffer_index = self.line_info[line_num as usize].buffer_index;
        let buffer_offset = self.line_info[line_num as usize].buffer_offset;

        if buffer_offset + line_offset as usize == self.buffers[buffer_index].len() {
            self.buffers[buffer_index].push(ch);
            
        } else {
            let rest_str = self.buffers[buffer_index].split_off(buffer_offset + line_offset as usize);
            self.buffers.insert(buffer_index+1, Box::new(String::from(rest_str)));
            self.buffers[buffer_index].push(ch);
        }
    }

    pub fn backspace_at(&mut self, line_offset: u32, line_num: u32) {
        let buffer_index = self.line_info[line_num as usize].buffer_index;
        let buffer_offset = self.line_info[line_num as usize].buffer_offset;
    }

    pub fn debug_print(&self) {
        for buffer in &self.buffers {
            print!("{}: ", buffer.capacity());
            println!("{}", buffer);
        }
    }
}