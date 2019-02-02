use crate::terminal::Terminal;

pub struct TextBuffer {
    line_num: u32,
    line_offset: u32,

    top_line: u32,
    left_col: u32,

    lines: Vec<Box<String>>,
    rendered_cached: Vec<Option<String>>,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        let textbuffer = TextBuffer {
            line_num: 0,
            line_offset: 0,
            top_line: 0,
            left_col: 0,
            lines: vec![Box::new(String::new())],
            rendered_cached: vec![None],
        };
        return textbuffer;
    }

    pub fn get_display_content(&self, width: u32, height: u32) -> Vec<String> {
        let mut return_content: Vec<String> = Vec::new();
        for i in 0..height {
            let index = (self.top_line+i) as usize;
            let line:String;
            if index < self.lines.len() {
                line = *self.lines[index].clone();
            } else {
                line = String::from("~");
            }
            return_content.push(line);
        }
        return return_content;
    }

    pub fn get_local_cursor(&self) -> (u16, u16) {
        let (cursor_x, cursor_y) = (self.line_offset - self.left_col+1, self.line_num - self.top_line+1);
        return (cursor_x as u16, cursor_y as u16);
    }
}

pub trait TextEditingModel {
    fn insert(&mut self, ch: char);
    fn backspace(&mut self);
    fn up(&mut self);
    fn down(&mut self);
    fn left(&mut self);
    fn right(&mut self);
}

impl TextEditingModel for TextBuffer {
    fn insert(&mut self, ch: char) {
        if ch == '\n' {
            self.lines.insert(self.line_num as usize + 1, Box::new(String::new()));
            self.line_num += 1;
            self.line_offset = 0;
        } else {
            self.lines[self.line_num as usize].insert(self.line_offset as usize, ch);
            self.line_offset += 1;
        }
    }

    fn backspace(&mut self) {
        if self.line_num == 0 && self.line_offset == 0 {
            return;
        }
        if self.line_offset > 0 {
            self.lines[self.line_num as usize].remove(self.line_offset as usize - 1);
            self.line_offset -= 1;
        } else {
            let left_content = self.lines[self.line_num as usize].clone();
            self.lines[self.line_num as usize -1].push_str(&left_content);
            self.lines.remove(self.line_num as usize);
            self.line_num -= 1;
            self.line_offset = self.lines[self.line_num as usize].len() as u32;
        }
    }

    fn up(&mut self) {
        if self.line_num <= 0 {
            return;
        }
        self.line_num -= 1;
        if self.line_offset > self.lines[self.line_num as usize].len()  as u32 {
            self.line_offset = self.lines[self.line_num as usize].len() as u32;
        }
    }

    fn down(&mut self) {
        if self.line_num >= self.lines.len() as u32 -1 {
            return;
        }
        self.line_num += 1;
        if self.line_offset > self.lines[self.line_num as usize].len()  as u32 {
            self.line_offset = self.lines[self.line_num as usize].len() as u32;
        }
    }

    fn left(&mut self) {
        if self.line_offset <= 0 {
            return;
        }
        self.line_offset -= 1;
    }

    fn right(&mut self) {
        if self.line_offset >= self.lines[self.line_num as usize].len()  as u32 {
            return;
        }
        self.line_offset += 1;
    }
}