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
    size: usize,
    style: Style,
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
    content: String,
    styles: Vec<StyleDescriptor>,
}

impl DisplayLine {
    pub fn new() -> DisplayLine {
        return DisplayLine {
            content: String::new(),
            styles: Vec::new(),
        };
    }

    pub fn from(content: String, ranges: Vec<(Style, &str)>) -> DisplayLine {
        let mut styles = Vec::new();
        for (style, substring) in ranges.iter() {
            let style_descriptor = StyleDescriptor::from(*style, substring.len());
            styles.push(style_descriptor);
        }
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

    pub fn set_content(&mut self, start_x: u16, start_y: u16, width: u16, height: u16, content: Vec<DisplayLine>, offset: usize) {
        /*
         * Warning: Performance is critical here!
         * Time matters more than memory.
         * 
         * Try to reduce loop time and 
         * the time write! macro is called.
         */
        write!(self.screen, "{}", termion::cursor::Hide).unwrap();
        for y in 0..height {
            self.set_cursor_pos(start_x, start_y+y);
            match content.get(y as usize) {
                Some(display_line) => {
                    let mut style_iter = display_line.styles.iter();
                    let mut style_descriptor: &StyleDescriptor;
                    let mut searched_end: usize = 0;

                    /*
                     * Load the initial style.
                     */ 
                    loop {
                        /*
                         * None indicate:
                         * - Null line with no descriptors.
                         * - Offset is still not found after iterating
                         *   all the style descriptors.
                         * 
                         * break it and nothing except for printing space till end
                         * would be done in the x loop.
                         */
                        style_descriptor = match style_iter.next() {
                            Some(sd) => sd,
                            None => break,
                        };
                        if searched_end <= offset && offset < searched_end + style_descriptor.size {
                            self.switch_style(style_descriptor.style);
                            searched_end += style_descriptor.size;
                            break
                        }
                        searched_end += style_descriptor.size;
                    }

                    /*
                     * searched_end:
                     *      now points to the first index that we don't have a style for it.
                     * style_descriptor:
                     *      now points to the current style_descriptor
                     */
                    for x in offset..offset+width as usize{
                        if let Some(c) = display_line.content.chars().nth(x as usize) {
                            if x as usize == searched_end {
                                /*
                                 * There definitely would be another style behind
                                 * when x == searched_end (which means goes beyond 
                                 * known range).
                                 * 
                                 * Otherwise the style and the content is not matched.
                                 */
                                if let Some(sd) = style_iter.next() {
                                    self.switch_style(sd.style);
                                    searched_end += sd.size;
                                } else {
                                    //write!(self.screen, "{}", " ".repeat(offset+width as usize-x)).unwrap();
                                    //break;
                                }
                            }
                            write!(self.screen, "{}", c).unwrap();
                        } else {
                            write!(self.screen, "{}", " ".repeat(offset+width as usize-x)).unwrap();
                            break;
                        }
                    }
                },
                None => {
                    write!(self.screen, "{}", " ".repeat(width as usize)).unwrap();
                },
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