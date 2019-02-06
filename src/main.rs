extern crate termion;
extern crate syntect;

mod terminal;
mod core;

use std::env;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Instant;

use crate::core::{TextBuffer, TextEditing, HighlightEngine, TextDisplay, FileRW};

use std::io::stdin;
use termion::event::{Key, Event, MouseEvent};
use termion::input::TermRead;

fn main() {
    let terminal = Arc::new(Mutex::new(terminal::Terminal::new()));
    let highlightengine = HighlightEngine::new();

    let args: Vec<String> = env::args().collect();

    let textbuffer = Arc::new(Mutex::new(TextBuffer::new()));
    let file_path = match args.get(1) {
        Some(path) => path.clone(),
        None => String::from(""),
    };

    {
        let mut t = textbuffer.lock().unwrap();
        t.set_file_path(file_path);
        t.load_file();
    }

    let (screen_refresh_send, screen_refresh_recv) = mpsc::channel();
    let highlight_refresh_sender = highlightengine.start_highlight(textbuffer.clone(), screen_refresh_send.clone());
    highlight_refresh_sender.send((0, 100)).unwrap();

    let terminal_clone = terminal.clone();
    let textbuffer_clone = textbuffer.clone();
    let screen_refresh_handle = thread::spawn(move || {
        for b in screen_refresh_recv {
            let print_time = Duration::from_micros(1000);
            if b == false {
                break;
            }
            let start = Instant::now();
            let mut t = terminal_clone.lock().unwrap();
            let mut tb = textbuffer_clone.lock().unwrap();
            let (t_width, t_height) = t.get_scale();
            tb.adjust_viewpoint(t_width as u32, t_height as u32);
            let display_lines = tb.get_display_lines(t_width as u32, t_height as u32);
            t.set_content(1, 1, t_width, t_height, display_lines);
            let print_time = start.elapsed();

            let (cursor_x, cursor_y) = tb.get_local_cursor();
            t.set_cursor_pos(cursor_x, cursor_y);
            print!("{:?}", print_time);
            t.flush();
        }
    });

    for e in stdin().events() {
        let evt = e.unwrap();
        let mut t = textbuffer.lock().unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Ctrl('q')  => break,
                    Key::Char(ch)   => t.insert(ch),
                    Key::Ctrl('a')  => t.head(),
                    Key::Ctrl('e')  => t.end(),
                    Key::Ctrl('u')  => t.del_to_head(),
                    Key::Ctrl('h')  => t.del_to_end(),
                    Key::Ctrl('b')  => t.left(),
                    Key::Ctrl('f')  => t.right(),
                    Key::Ctrl('p')  => t.up(),
                    Key::Ctrl('n')  => t.down(),
                    Key::Ctrl('s')  => t.save_file(),
                    Key::Backspace  => t.backspace(),
                    Key::Up         => t.up(),
                    Key::Down       => t.down(),
                    Key::Left       => t.left(),
                    Key::Right      => t.right(),
                    _ => {},
                }
            },
            _ => {},
        };
        screen_refresh_send.clone().send(true).unwrap();
        highlight_refresh_sender.clone().send((match t.line_num {
            0 => 0,
            l => l-1,
        } as u32, t.top_line+100)).unwrap();
    }

    screen_refresh_send.clone().send(false).unwrap();
    screen_refresh_handle.join().unwrap();
    { terminal.lock().unwrap().finish() };


}
