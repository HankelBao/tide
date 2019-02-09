extern crate termion;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::terminal_size;
use std::io::{Write, stdout, stdin, Stdin, Stdout};
use crate::core::{Style, FontStyle};

pub use termion::event::{Key, Event, MouseEvent};
pub use termion::input::TermRead;

pub struct Terminal {
    /*
     * The main event loop should have the
     * ownership of stdin, so stdin is not
     * restored here.
     */
    screen: MouseTerminal<RawTerminal<Stdout>>,
    width: u16,
    height: u16,
}

#[derive(Clone, Debug)]
pub struct StyleDescriptor {
    pub size: usize,
    pub style: Style,
}

impl StyleDescriptor {
    pub fn from(style: Style, size: usize) -> StyleDescriptor {
        return StyleDescriptor {
            size,
            style,
        };
    }
}

#[derive(Clone, Debug)]
pub struct DisplayLine {
    pub content: String,
    pub styles: Vec<StyleDescriptor>,
}

impl DisplayLine {
    pub fn new() -> DisplayLine {
        return DisplayLine {
            content: String::new(),
            styles: Vec::new(),
        };
    }

    pub fn from(content: String, style_descriptors: Vec<StyleDescriptor>) -> DisplayLine {
        return DisplayLine {
            content,
            styles: style_descriptors,
        };
    }
}

impl Terminal {
    pub fn new() -> Terminal {
        let mut terminal = Terminal {
            screen: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
            width: 0,
            height: 0,
        };
        terminal.update_scale();
        terminal.set_cursor_pos(1, 1);
        terminal.clear();
        terminal.flush();
        return terminal;
    }

    pub fn get_stdin(&self) -> Stdin {
        return stdin()
    }

    pub fn clear(&mut self) {
        write!(self.screen, "{}", termion::clear::All).unwrap();
    }

    fn update_scale(&mut self) {
        let (width, height) = terminal_size().unwrap();
        self.width = width;
        self.height = height;
    }

    pub fn get_scale(&mut self) -> (u16, u16) {
        self.update_scale();
        return (self.width, self.height);
    }

    fn switch_style(&mut self, style: Style) {
        write!(self.screen, "{}", termion::color::Fg(termion::color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b))).unwrap();
        write!(self.screen, "{}", termion::color::Bg(termion::color::Rgb(style.background.r, style.background.g, style.background.b))).unwrap();
        match style.font_style {
            FontStyle::BOLD => write!(self.screen, "{}", termion::style::Bold).unwrap(),
            FontStyle::UNDERLINE => write!(self.screen, "{}", termion::style::Underline).unwrap(),
            FontStyle::ITALIC => write!(self.screen, "{}", termion::style::Italic).unwrap(),
            _ => {},
        };
    }

    pub fn set_content(&mut self, start_x: u16, start_y: u16, width: u16, height: u16, content: Vec<DisplayLine>) {
        /*
         * Warning: Performance is critical here!
         * Time matters more than memory.
         *
         * Try to reduce loop time and
         * the time write! macro is called.
         */
        write!(self.screen, "{}", termion::cursor::Save).unwrap();
        write!(self.screen, "{}", termion::cursor::Hide).unwrap();
        for y in 0..height {
            self.set_cursor_pos(start_x, start_y+y);
            match content.get(y as usize) {
                Some(display_line) => {
                    let mut current_style_start: usize = 0;
                    for style_descriptor in display_line.styles.iter() {
                        self.switch_style(style_descriptor.style);
                        let content: String = display_line.content.chars().skip(current_style_start).take(style_descriptor.size).collect();
                        write!(self.screen, "{}", content).unwrap();
                        current_style_start += style_descriptor.size;
                    }

                    let content_len = display_line.content.len();
                    if current_style_start < content_len {
                        let content: String = display_line.content.chars().skip(current_style_start).take(content_len - current_style_start).collect();
                        write!(self.screen, "{}", content).unwrap();
                    }

                    if content_len < width as usize {
                        write!(self.screen, "{}", " ".repeat(width as usize - content_len)).unwrap();
                    }

                },
                None => {
                    write!(self.screen, "{}", " ".repeat(width as usize)).unwrap();
                },
            }
        }
        write!(self.screen, "{}", termion::cursor::Restore).unwrap();
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
    }

    pub fn set_cursor_pos(&mut self, x: u16, y: u16) {
        if x == 0 || y == 0 {
            panic!("There shouldn't be 0 in cursor_pos");
        }
        write!(self.screen, "{}", termion::cursor::Goto(x, y)).unwrap();
    }

    pub fn flush(&mut self) {
        self.screen.flush().unwrap()
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
