use crate::terminal::{DisplayLine, StyleDescriptor};
use super::TextBuffer;

pub trait TextDisplay {
    fn adjust_viewpoint(&mut self, width: u32, height: u32);
    fn get_display_lines(&self, width: u32, height: u32) -> Vec<DisplayLine>;
    fn get_local_cursor(&self) -> (u16, u16);
}

impl TextDisplay for TextBuffer {
    fn adjust_viewpoint(&mut self, width: u32, height: u32) {
        self.view_height = height;
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

    fn get_display_lines(&self, width: u32, height: u32) -> Vec<DisplayLine> {
        let mut return_content: Vec<DisplayLine> = Vec::new();
        let lines = self.lines.lock().unwrap();
        for i in 0..height {
            let index = (self.top_line+i) as usize;
            if index >= lines.len() {
                break;
            }
            let line:DisplayLine;
            let offset = self.left_col as usize;

            let content: String = lines[index].content().chars().skip(offset).take(width as usize).collect();

            let mut styles: Vec<StyleDescriptor> = Vec::new();
            let mut current_style_start: usize = 0;
            for style in lines[index as usize].styles_cache.iter() {
                if current_style_start + style.size < offset || current_style_start > offset+width as usize {
                    continue;
                }
                let mut cloned_style = style.clone();
                if current_style_start <= offset && offset < current_style_start+style.size {
                    cloned_style.size -= offset - current_style_start;
                }
                if current_style_start <= (offset + width as usize) && (offset + width as usize) < current_style_start+style.size {
                    cloned_style.size -= current_style_start+style.size-offset;
                }
                current_style_start += style.size;
                styles.push(cloned_style);
            }

            line = DisplayLine {
                content,
                styles,
            };
            return_content.push(line);
        }
        return return_content;
    }

    fn get_local_cursor(&self) -> (u16, u16) {
        let (cursor_x, cursor_y) = (self.line_offset - self.left_col, self.line_num - self.top_line);
        return (cursor_x as u16, cursor_y as u16);
    }
}