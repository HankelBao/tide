use crate::terminal::DisplayLine;
use crate::terminal::Terminal;
use crate::core::Style;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct View {
    terminal: Rc<RefCell<Terminal>>,
    /*
     * Pub because ViewManager gonna change it.
     */
    pub start_x: u16,
    pub start_y: u16,
    pub width: u16,
    pub height: u16,
}

impl View {
    pub fn new(terminal: Rc<RefCell<Terminal>>) -> View {
        let view = View {
            terminal,
            start_x: 0,
            start_y: 0,
            width: 0,
            height: 0,
        };
        view
    }

    pub fn with_width(terminal: Rc<RefCell<Terminal>>, width: u16) -> View {
        let mut view = View::new(terminal);
        view.width = width;
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

    pub fn set_content(&self, display_lines: Vec<DisplayLine>, default_style: Style) {
        let mut terminal = self.terminal.borrow_mut();
        terminal.set_content(self.start_x, self.start_y, self.width, self.height, display_lines, default_style);
    }

    pub fn set_cursor(&self, x: u16, y: u16) {
        let mut terminal = self.terminal.borrow_mut();
        terminal.set_cursor_pos(x+self.start_x, y+self.start_y);
    }

    pub fn flush(&self) {
        let mut terminal = self.terminal.borrow_mut();
        terminal.flush();
    }
}
