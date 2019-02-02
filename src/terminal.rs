extern crate termion;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::terminal_size;
use std::io::{Write, stdout, stdin, Stdin, Stdout};

pub use termion::event::{Key, Event, MouseEvent};
pub use termion::input::TermRead;

pub struct Terminal {
    /*
     * The main event loop should have the
     * ownership of stdin, so stdin is not 
     * restored here.
     */
    stdout: MouseTerminal<RawTerminal<Stdout>>,
    width: u16,
    height: u16,
}

pub fn new() -> Terminal {
    return Terminal::new()
}

impl Terminal {
    pub fn new() -> Terminal {
        let mut terminal = Terminal {
            stdout: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
            width: 0,
            height: 0,
        };
        terminal.update_scale();
        terminal.set_cursor_pos(1, 1);
        terminal.clear();
        terminal.flush();
        return terminal;
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    fn update_scale(&mut self) {
        let (width, height) = terminal_size().unwrap();
        self.width = width;
        self.height = height;
    }

    pub fn get_scale(&self) -> (u16, u16) {
        return (self.width, self.height);
    }

    pub fn set_content(&mut self, start_x: u16, start_y: u16, width: u16, height: u16, content: Vec<String>) {
        write!(self.stdout, "{}", termion::cursor::Hide).unwrap();
        self.set_cursor_pos(1, 1);
        for y in 0..height {
            for x in 0..width {
                let char_to_draw = match content.get(y as usize) {
                    Some(line) => {
                        match line.chars().nth(x as usize) {
                            Some(c) => c,
                            None => ' ',
                        }
                    },
                    None => ' ',
                };
                write!(self.stdout, "{}", char_to_draw).unwrap();
            }
        }
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }

    pub fn set_cursor_pos(&mut self, x: u16, y: u16) {
        if x == 0 || y == 0 {
            panic!("There shouldn't be 0 in cursor_pos");
        }
        write!(self.stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap()
    }

    pub fn set_content_at(&mut self, x: u16, y: u16, content: char) {
        self.set_cursor_pos(x, y);
        write!(self.stdout, "{}", content).unwrap();
    }

    pub fn get_events(&mut self) -> termion::input::Events<Stdin> {
        return stdin().events();
    }

    pub fn finish(&mut self) {
        /*
         * We have the set the cursor back to
         * the origin. Otherwise, there would be
         * a % when the program exits and the 
         * printing history would be left.
         */
        self.set_cursor_pos(1, 1);
        self.flush();
    }
}