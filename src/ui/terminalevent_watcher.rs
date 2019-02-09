use crate::core::{Message, MessageSender};
use crate::terminal::Terminal;
use crate::terminal::{Event, Key};
use termion::input::TermRead;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct TerminalEventWatcher {
    messagesender: MessageSender,
    terminal: Arc<Mutex<Terminal>>,
}

impl TerminalEventWatcher {
    pub fn new(messagesender: MessageSender, terminal: Arc<Mutex<Terminal>>) -> TerminalEventWatcher {
        return TerminalEventWatcher {
            messagesender,
            terminal,
        }
    }

    pub fn start_watch_thread(&self) {
        let messagesender = self.messagesender.clone();
        let terminal = self.terminal.clone();
        thread::spawn(move || {
            let stdin = {terminal.lock().unwrap().get_stdin()};
            for e in stdin.events() {
                let evt = e.unwrap();
                match evt {
                    Event::Key(key) => {
                        match key {
                            Key::Ctrl('q')  => { messagesender.send(Message::Quit).unwrap() },
                            _ => { messagesender.send(Message::TerminalKey(key)).unwrap(); },
                        }
                    },
                    _ => {},
                };
            }
        });
    }
}
