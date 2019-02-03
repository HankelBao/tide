use super::TextBuffer;


pub trait TextEditing {
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

impl<'a> TextEditing for TextBuffer<'a> {
    fn insert(&mut self, ch: char) {
        match ch {
            '\n' => {
                let left_content = self.lines[self.line_num as usize].split_off(self.line_offset as usize);
                self.lines.insert(self.line_num as usize + 1, Box::new(left_content));
                self.line_num += 1;
                self.line_offset = 0;
            },
            '\t' => {
                let target_index = ((self.line_offset / 4) as u32 + 1) * 4;
                for _ in self.line_offset..target_index {
                    self.insert(' ');
                }
            },
            _ => {
                self.lines[self.line_num as usize].insert(self.line_offset as usize, ch);
                self.line_offset += 1;
            },
        }
    }

    fn backspace(&mut self) {
        if self.line_num == 0 && self.line_offset == 0 {
            return;
        }
        if self.line_offset > 0 {
            if self.lines[self.line_num as usize].chars().nth(self.line_offset as usize-1).unwrap() != ' ' {
                self.lines[self.line_num as usize].remove(self.line_offset as usize - 1);
                self.line_offset -= 1;
                return
            }
            let target_index = ((self.line_offset-1) /4 as u32) * 4;
            for i in (target_index..self.line_offset).rev() {
                if self.lines[self.line_num as usize].chars().nth(i as usize).unwrap() != ' ' {
                    return
                }
                self.lines[self.line_num as usize].remove(self.line_offset as usize - 1);
                self.line_offset -= 1;
            }
        } else {
            let left_content = self.lines[self.line_num as usize].clone();
            self.line_offset = self.lines[self.line_num as usize-1].len() as u32;
            self.lines[self.line_num as usize -1].push_str(&left_content);
            self.lines.remove(self.line_num as usize);
            self.line_num -= 1;
        }
    }

    fn goto_line(&mut self, mut line_num: u32) {
        /*
         * line_num must be larger or equal to 0,
         * because of its type.
         */
        if line_num > self.lines.len() as u32 - 1 {
            line_num = self.lines.len() as u32 -1;
        }
        self.line_num = line_num;
        if self.line_offset > self.lines[self.line_num as usize].len()  as u32 {
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

    fn head(&mut self) {
        self.line_offset = 0;
    }

    fn end(&mut self) {
        self.line_offset = self.lines[self.line_num as usize].len() as u32;
    }

    fn del_to_head(&mut self) {
        let line = self.lines[self.line_num as usize].clone();
        let (_, right) = line.split_at(self.line_offset as usize);
        self.lines[self.line_num as usize] = Box::new(String::from(right));
        self.line_offset = 0;
    }

    fn del_to_end(&mut self) {
        let line = self.lines[self.line_num as usize].clone();
        let (left, _) = line.split_at(self.line_offset as usize);
        self.lines[self.line_num as usize] = Box::new(String::from(left));
        self.line_offset = self.lines[self.line_num as usize].len() as u32;
    }
}