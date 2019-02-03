extern crate termion;
extern crate syntect;

mod terminal;
mod core;

use std::env;
use std::time::Instant;

use crate::core::{TextBuffer, TextEditing, HighlightEngine, SyntaxHighlight, TextDisplay, FileRW};
use crate::terminal::{Event, Key};

fn main() {
    let mut terminal = terminal::Terminal::new();
    let highlightengine = HighlightEngine::new();

    let args: Vec<String> = env::args().collect();

    let mut textbuffer = TextBuffer::new(highlightengine.get_syntax_plain_text());
    let file_path = match args.get(1) {
        Some(path) => path.clone(),
        None => String::from(""),
    };
    textbuffer.set_file_path(file_path);
    textbuffer.load_file();

    textbuffer.update_syntax(&highlightengine);

    let (t_width, t_height) = terminal.get_scale();
    textbuffer.adjust_viewpoint(t_width as u32, t_height as u32);
    textbuffer.highlight(&highlightengine);
    terminal.set_content(1, 1, t_width, t_height, textbuffer.get_display_lines(t_width as u32, t_height as u32), textbuffer.left_col);

    let (cursor_x, cursor_y) = textbuffer.get_local_cursor();
    terminal.set_cursor_pos(cursor_x, cursor_y);
    terminal.flush();

    for e in terminal.get_events() {
        let start = Instant::now();
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
                    Key::Ctrl('p') => textbuffer.up(),
                    Key::Ctrl('n') => textbuffer.down(),
                    Key::Ctrl('s') => textbuffer.save_file(),
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
        let operation_time = start.elapsed();
        let (t_width, t_height) = terminal.get_scale();
        textbuffer.adjust_viewpoint(t_width as u32, t_height as u32);
        textbuffer.highlight(&highlightengine);
        let highlight_time = start.elapsed();
        terminal.set_content(1, 1, t_width, t_height, textbuffer.get_display_lines(t_width as u32, t_height as u32), textbuffer.left_col);
        let content_time = start.elapsed();

        let (cursor_x, cursor_y) = textbuffer.get_local_cursor();
        terminal.set_cursor_pos(cursor_x, cursor_y);
        terminal.flush();
        
        print!(" {:?}", content_time - highlight_time);
        terminal.flush();
    }

    terminal.finish();

}
