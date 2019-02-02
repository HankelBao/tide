extern crate termion;

mod terminal;
mod textbuffer;

use textbuffer::{TextBuffer, TextEditingModel};

use std::time::Duration;
use std::time::Instant;
use std::thread;

use std::io::stdin;
use crate::terminal::{Event, Key};

fn main() {
    let mut terminal = terminal::new();
    let mut textbuffer = textbuffer::TextBuffer::new();

    for e in terminal.get_events() {
        let evt = e.unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Ctrl('q') => break,
                    Key::Char(ch) => {
                        textbuffer.insert(ch);
                    },
                    Key::Backspace => {
                        textbuffer.backspace();
                    },
                    Key::Up => {
                        textbuffer.up();
                    },
                    Key::Down => {
                        textbuffer.down();
                    },
                    Key::Left => {
                        textbuffer.left();
                    },
                    Key::Right => {
                        textbuffer.right();
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        let (t_width, t_height) = terminal.get_scale();
        terminal.set_content(1, 1, t_width, t_height, textbuffer.get_display_content(t_width as u32, t_height as u32));

        let (cursor_x, cursor_y) = textbuffer.get_local_cursor();
        terminal.set_cursor_pos(cursor_x, cursor_y);
        terminal.flush();
    }


    terminal.finish();


}
