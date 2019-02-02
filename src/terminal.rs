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

#[derive(Clone)]
pub struct StyleDescriptor {
    size: usize,
    style: Style,
}

impl StyleDescriptor {
    pub fn new(size: usize, style: Style) -> StyleDescriptor {
        return StyleDescriptor {
            size,
            style,
        };
    }
}

#[derive(Clone)]
pub struct DisplayLine {
    content: String,
    styles: Vec<StyleDescriptor>,
}

impl DisplayLine {
    pub fn new(content: String, styles: Vec<StyleDescriptor>) -> DisplayLine {
        return DisplayLine {
            content,
            styles,
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

    pub fn clear(&mut self) {
        write!(self.screen, "{}", termion::clear::All).unwrap();
    }

    fn update_scale(&mut self) {
        let (width, height) = terminal_size().unwrap();
        self.width = width;
        self.height = height;
    }

    pub fn get_scale(&self) -> (u16, u16) {
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

    pub fn set_content(&mut self, start_x: u16, start_y: u16, width: u16, height: u16, content: Vec<DisplayLine>, offset: u32) {
        write!(self.screen, "{}", termion::cursor::Hide).unwrap();
        for y in 0..height {
            self.set_cursor_pos(start_x, start_y+y);
            for x in offset..offset+width as u32{
                let char_to_draw = match content.get(y as usize) {
                    Some(display_line) => {
                        match display_line.content.chars().nth(x as usize) {
                            Some(c) => {
                                let mut dividing_point: usize = 0;
                                for style_descriptor in &display_line.styles {
                                    if x as usize == dividing_point {
                                        self.switch_style(style_descriptor.style);
                                    }
                                    dividing_point += style_descriptor.size;
                                }
                                c
                            },
                            None => ' ',
                        }
                    },
                    None => ' ',
                };
                write!(self.screen, "{}", char_to_draw).unwrap();
            }
        }
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