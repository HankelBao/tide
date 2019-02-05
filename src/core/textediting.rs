use super::TextBuffer;
use super::TextLine;


pub trait TextEditing {
    /*
     * Warning:
     * Since any operation about lines will lock itself.
     * Functions here should never call any function here.
     */
    fn insert(&mut self, ch: char);
    fn backspace(&mut self);
    fn goto_line(&mut self, line_num: u32);
    fn up(&mut self);
    fn down(&mut self);
    fn left(&mut self);
    fn right(&mut self);
    fn head(&mut self);
    fn end(&mut self);
    fn del_to_head(&mut self);
    fn del_to_end(&mut self);
}

impl TextEditing for TextBuffer {
    fn insert(&mut self, ch: char) {
        let mut lines = self.lines.lock().unwrap();
        match ch {
            '\n' => {
                let left_content = lines[self.line_num as usize].split_off(self.line_offset as usize);
                lines.insert(self.line_num as usize + 1, Box::new(TextLine::from(left_content)));
                self.line_num += 1;
                self.line_offset = 0;
            },
            '\t' => {
                let target_index = ((self.line_offset / 4) as u32 + 1) * 4;
                for _ in self.line_offset..target_index {
                    lines[self.line_num as usize].insert(self.line_offset as usize, ch);
                    self.line_offset += 1;
                }
            },
            _ => {
                lines[self.line_num as usize].insert(self.line_offset as usize, ch);
                self.line_offset += 1;
            },
        }
    }

    fn backspace(&mut self) {
        let mut lines = self.lines.lock().unwrap();
        if self.line_num == 0 && self.line_offset == 0 {
            return;
        }
        if self.line_offset > 0 {
            if lines[self.line_num as usize].char_at(self.line_offset as usize-1) != ' ' {
                lines[self.line_num as usize].remove(self.line_offset as usize - 1);
                self.line_offset -= 1;
                return
            }
            let target_index = ((self.line_offset-1) /4 as u32) * 4;
            for i in (target_index..self.line_offset).rev() {
                if lines[self.line_num as usize].char_at(i as usize) != ' ' {
                    return
                }
                lines[self.line_num as usize].remove(self.line_offset as usize - 1);
                self.line_offset -= 1;
            }
        } else {
            let left_content = lines[self.line_num as usize].content();
            self.line_offset = lines[self.line_num as usize-1].len() as u32;
            lines[self.line_num as usize -1].push_str(left_content);
            lines.remove(self.line_num as usize);
            self.line_num -= 1;
        }
    }

    fn goto_line(&mut self, mut line_num: u32) {
        /*
         * line_num must be larger or equal to 0,
         * because of its type.
         */
        let mut lines = self.lines.lock().unwrap();
        if line_num > lines.len() as u32 - 1 {
            line_num = lines.len() as u32 -1;
        }
        self.line_num = line_num;
        if self.line_offset > lines[self.line_num as usize].len()  as u32 {
            self.line_offset = lines[self.line_num as usize].len() as u32;
        }
    }

    fn up(&mut self) {
        let mut lines = self.lines.lock().unwrap();
        if self.line_num <= 0 {
            return;
        }
        self.line_num -= 1;
        if self.line_offset > lines[self.line_num as usize].len()  as u32 {
            self.line_offset = lines[self.line_num as usize].len() as u32;
        }
    }

    fn down(&mut self) {
        let mut lines = self.lines.lock().unwrap();
        if self.line_num >= lines.len() as u32 -1 {
            return;
        }
        self.line_num += 1;
        if self.line_offset > lines[self.line_num as usize].len()  as u32 {
            self.line_offset = lines[self.line_num as usize].len() as u32;
        }
    }

    fn left(&mut self) {
        if self.line_offset <= 0 {
            return;
        }
        self.line_offset -= 1;
    }

    fn right(&mut self) {
        let mut lines = self.lines.lock().unwrap();
        if self.line_offset >= lines[self.line_num as usize].len()  as u32 {
            return;
        }
        self.line_offset += 1;
    }

    fn head(&mut self) {
        self.line_offset = 0;
    }

    fn end(&mut self) {
        let mut lines = self.lines.lock().unwrap();
        self.line_offset = lines[self.line_num as usize].len() as u32;
    }

    fn del_to_head(&mut self) {
        let mut lines = self.lines.lock().unwrap();
        let line = lines[self.line_num as usize].content();
        let (_, right) = line.split_at(self.line_offset as usize);
        lines[self.line_num as usize] = Box::new(TextLine::from(right.to_string()));
        self.line_offset = 0;
    }

    fn del_to_end(&mut self) {
        let mut lines = self.lines.lock().unwrap();
        let line = lines[self.line_num as usize].content();
        let (left, _) = line.split_at(self.line_offset as usize);
        lines[self.line_num as usize] = Box::new(TextLine::from(left.to_string()));
        self.line_offset = lines[self.line_num as usize].len() as u32;
    }
}