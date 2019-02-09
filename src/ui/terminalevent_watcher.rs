use crate::core::{Message, MessageSender};
use crate::terminal::{Event, Key};
use termion::input::TermRead;
use termion::terminal_size;
use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct TerminalEventWatcher {
    messagesender: MessageSender,
}

impl TerminalEventWatcher {
    pub fn new(messagesender: MessageSender) -> TerminalEventWatcher {
        return TerminalEventWatcher {
            messagesender,
        }
    }

    pub fn start_watch_thread(&self) {
        let messagesender = self.messagesender.clone();
        thread::spawn(move || {
            let stdin = stdin();
            for e in stdin.events() {
                let evt = e.unwrap();
                match evt {
                    Event::Key(key) => {
                        match key {
                            Key::Ctrl('q')  => { messagesender.send(Message::Quit).unwrap(); break },
                            _ => { messagesender.send(Message::TerminalKey(key)).unwrap(); },
                        }
                    },
                    _ => {},
                };
            }
        });
        let messagesender = self.messagesender.clone();
        thread::spawn(move || {
            let (mut t_width, mut t_height) = (0, 0);
            loop {
                if let Ok((width, height)) = terminal_size() {
                    if width != t_width || height != t_height {
                        t_width = width;
                        t_height = height;
                        messagesender.send(Message::TerminalResize(t_width, t_height)).unwrap();
                    }
                }

            }
        });
    }

}
