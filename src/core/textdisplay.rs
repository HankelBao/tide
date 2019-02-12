use crate::terminal::{DisplayLine, StyleDescriptor};
use super::TextBuffer;

pub trait TextDisplay {
    fn adjust_viewpoint(&mut self, width: u32, height: u32);
    fn get_display_lines(&mut self, width: u32, height: u32) -> Vec<DisplayLine>;
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

    fn get_display_lines(&mut self, width: u32, height: u32) -> Vec<DisplayLine> {
        /*
         * Display Lines are divided into two parts:
         * - line_num: the section to display line numbers,
         *      the width is decided by digit of the largest_line_num
         *      with a space after it.     
         * - code: the actual code in the view
         *      styles here would be rearranged to match the
         *      the left_col and width of the view.
         */


        let mut return_content: Vec<DisplayLine> = Vec::new();
        let lines = self.lines.lock().unwrap();


        /*
         * Calculate the view_line_num width and 
         */
        let largest_line_num_on_view = if lines.len() as u32 > self.top_line + height {
            self.top_line + height
        } else {
            lines.len() as u32
        };
        let largest_line_num_digits = largest_line_num_on_view.to_string().chars().count();
        let view_line_num_width = largest_line_num_digits + 4; // 1 represents the space.
        let view_code_width = width - self.view_line_num_width as u32;

        /*
         * Record the line_num_width
         */
        self.view_line_num_width = view_line_num_width;

        for i in 0..height {
            let index = (self.top_line+i) as usize;
            if index >= lines.len() {
                break;
            }
            let offset = self.left_col as usize;

            // Line number
            let content_line_num: String = format!(" {:width$}   ", index+1, width=largest_line_num_digits);
            let mut styles_line_num = vec![StyleDescriptor::from(self.line_num_style.clone(), view_line_num_width)];

            // Code
            let content_code: String = lines[index].content().chars().skip(offset).take(view_code_width as usize).collect();
            let mut styles_code: Vec<StyleDescriptor> = Vec::new();

            let mut current_style_start: usize = 0;
            for style in lines[index as usize].styles_cache.iter() {
                if current_style_start + style.size < offset || current_style_start > offset+view_code_width as usize {
                    continue;
                }
                let mut cloned_style = style.clone();
                if current_style_start <= offset && offset < current_style_start+style.size {
                    cloned_style.size -= offset - current_style_start;
                }
                if current_style_start <= (offset + view_code_width as usize) && (offset + view_code_width as usize) < current_style_start+style.size {
                    cloned_style.size -= current_style_start+style.size-(offset+view_code_width as usize);
                }
                current_style_start += style.size;
                styles_code.push(cloned_style);
            }

            // Combine
            let line = DisplayLine {
                content: content_line_num+&content_code,
                styles: {
                    /*
                     * Styles line number is no longer going to be useful here.
                     * It could be changed without problems.
                     */
                    styles_line_num.extend(styles_code);
                    styles_line_num
                }
            };
            return_content.push(line);
        }
        return return_content;
    }

    fn get_local_cursor(&self) -> (u16, u16) {
        let (cursor_x, cursor_y) = (self.line_offset - self.left_col + self.view_line_num_width as u32, self.line_num - self.top_line);
        return (cursor_x as u16, cursor_y as u16);
    }
}
