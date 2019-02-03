use crate::terminal::DisplayLine;
use super::TextBuffer;

pub trait TextDisplay {
    fn adjust_viewpoint(&mut self, width: u32, height: u32);
    fn get_display_lines(&self, width: u32, height: u32) -> Vec<DisplayLine>;
    fn get_local_cursor(&self) -> (u16, u16);
}

impl<'a> TextDisplay for TextBuffer<'a> {
    fn adjust_viewpoint(&mut self, width: u32, height: u32) {
        if self.top_line > self.line_num {
            self.top_line = self.line_num;
        }
        if self.top_line + height - 1 < self.line_num {
            self.top_line = self.line_num - height + 1;
        }
        if self.left_col > self.line_offset {
            self.left_col = self.line_offset;
        }
        if self.left_col + width - 1 < self.line_offset {
            self.left_col = self.line_offset - width + 1;
        }
    }

    fn get_display_lines(&self, _width: u32, height: u32) -> Vec<DisplayLine> {
        let mut return_content: Vec<DisplayLine> = Vec::new();
        for i in 0..height {
            let index = (self.top_line+i) as usize;
            let line:DisplayLine;
            if index < self.lines.len() {
                line = self.rendered_cache[index as usize].clone();
            } else {
                break;
            }
            return_content.push(line);
        }
        return return_content;
    }

    fn get_local_cursor(&self) -> (u16, u16) {
        let (cursor_x, cursor_y) = (self.line_offset - self.left_col+1, self.line_num - self.top_line+1);
        return (cursor_x as u16, cursor_y as u16);
    }
}