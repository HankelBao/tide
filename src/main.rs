extern crate termion;
extern crate syntect;

mod terminal;
mod core;

use core::{TextBuffer, TextEditing, HighlightEngine, SyntaxHighlight, TextDisplay};

use crate::terminal::{Event, Key};

fn main() {
    let mut terminal = terminal::Terminal::new();
    let mut textbuffer = TextBuffer::new();
    let highlightengine = HighlightEngine::new();

    for e in terminal.get_events() {
        let evt = e.unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Char(ch) => textbuffer.insert(ch),
                    Key::Ctrl('q') => break,
                    Key::Ctrl('a') => textbuffer.head(),
                    Key::Ctrl('e') => textbuffer.end(),
                    Key::Ctrl('u') => textbuffer.del_to_head(),
                    Key::Ctrl('h') => textbuffer.del_to_end(),
                    Key::Ctrl('b') => textbuffer.left(),
                    Key::Ctrl('f') => textbuffer.right(),
                    Key::Backspace => textbuffer.backspace(),
                    Key::Up => textbuffer.up(),
                    Key::Down => textbuffer.down(),
                    Key::Left => textbuffer.left(),
                    Key::Right => textbuffer.right(),
                    _ => {},
                }
            },
            _ => {},
        };
        let (t_width, t_height) = terminal.get_scale();
        textbuffer.adjust_viewpoint(t_width as u32, t_height as u32);
        textbuffer.highlight(&highlightengine.ps, &highlightengine.ts);
        terminal.set_content(2, 1, t_width-1, t_height, textbuffer.get_display_lines(t_width as u32-1, t_height as u32), textbuffer.left_col);

        let (cursor_x, cursor_y) = textbuffer.get_local_cursor();
        terminal.set_cursor_pos(cursor_x+1, cursor_y);
        terminal.flush();
    }

    terminal.finish();

}
