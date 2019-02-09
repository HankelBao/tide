use crate::terminal::DisplayLine;
use crate::terminal::Terminal;

use std::sync::{Arc, Mutex};

pub type SyncView = Arc<Mutex<View>>;

pub struct View {
    terminal: Arc<Mutex<Terminal>>,
    start_x: u16,
    start_y: u16,
    width: u16,
    height: u16,
}

impl View {
    pub fn new(terminal: Arc<Mutex<Terminal>>) -> View {
        let view = View {
            terminal,
            start_x: 0,
            start_y: 0,
            width: 0,
            height: 0,
        };
        view
    }

    pub fn from(terminal: Arc<Mutex<Terminal>>, start_x: u16, start_y: u16, width: u16, height: u16) -> View {
        let view = View {
            terminal,
            start_x,
            start_y,
            width,
            height,
        };
        view
    }

    pub fn get_scale(&self) -> (u16, u16) {
        return (self.width, self.height)
    }

    pub fn get_width(&self) -> u16 {
        return self.width
    }

    pub fn get_height(&self) -> u16 {
        return self.height
    }

    pub fn set_content(&self, display_lines: Vec<DisplayLine>) {
        let mut terminal = self.terminal.lock().unwrap();
        terminal.set_content(self.start_x, self.start_y, self.width, self.height, display_lines);
    }

    pub fn set_cursor(&self, x: u16, y: u16) {
        let mut terminal = self.terminal.lock().unwrap();
        terminal.set_cursor_pos(x+self.start_x, y+self.start_y);
    }

    pub fn flush(&self) {
        let mut terminal = self.terminal.lock().unwrap();
        terminal.flush();
    }
}
