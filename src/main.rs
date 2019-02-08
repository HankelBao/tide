extern crate termion;
extern crate syntect;

mod components;
mod terminal;
mod core;
mod ui;

use std::env;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

use crate::core::{HighlightEngine};
use crate::components::TextEditor;
use crate::components::Statusline;
use crate::ui::ViewManager;

use std::io::stdin;
use termion::event::{Key, Event};
use termion::input::TermRead;

fn main() {
    let terminal = Arc::new(Mutex::new(terminal::Terminal::new()));
    let viewmanager = ViewManager::new(terminal.clone());
    let main_view = viewmanager.main_view.clone();
    let highlightengine = HighlightEngine::new();

    let args: Vec<String> = env::args().collect();

    let (statusline_display_send, statusline_display_recv) = mpsc::channel();

    let mut texteditor: TextEditor = match args.get(1) {
        Some(path) => TextEditor::new_with_file(main_view.clone(), &highlightengine, path.clone(), statusline_display_send.clone()),
        None => TextEditor::new(main_view.clone(), &highlightengine, statusline_display_send.clone()),
    };

    let mut statusline: Statusline = Statusline::new(viewmanager.statusline_view.clone(), &highlightengine, statusline_display_recv);
    statusline.start_display_thread();

    texteditor.start_display_thread();

    for e in stdin().events() {
        let evt = e.unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Ctrl('q')  => break,
                    _ => {texteditor.key(key)},
                }
            },
            _ => {},
        };
    }

    {
        let mut t = terminal.lock().unwrap();
        t.finish();
        drop(t);
    };
}
