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
use std::time::Instant;
use std::io::stdin;
use std::rc::Rc;
use std::cell::RefCell;

use crate::core::{HighlightEngine};
use crate::core::Message;
use crate::core::MessageManager;
use crate::components::TextEditor;
use crate::components::Statusline;
use crate::components::ProjectTree;
use crate::ui::ViewManager;
use crate::ui::{UIComponent, UISelector};
use crate::ui::SelectorManager;
use crate::ui::TerminalEventWatcher;

use termion::event::{Key, Event};

fn main() {
    let highlightengine = HighlightEngine::new(String::from("base16-ocean.dark"));

    let terminal = Arc::new(Mutex::new(terminal::Terminal::new()));
    let viewmanager = ViewManager::new(terminal.clone());

    let (mut messagemanager, message_recv) = MessageManager::new();

    let args: Vec<String> = env::args().collect();

    let mut texteditor: Rc<RefCell<TextEditor>> = Rc::new(RefCell::new(match args.get(1) {
        Some(path) => TextEditor::new_with_file(
            messagemanager.get_messagesender(),
            viewmanager.main_view.clone(),
            &highlightengine,
            path.clone()),
        None => TextEditor::new(
            messagemanager.get_messagesender(),
            viewmanager.main_view.clone(),
            &highlightengine),
    }));

    let mut statusline: Rc<RefCell<Statusline>> = Rc::new(RefCell::new(Statusline::new(
        messagemanager.get_messagesender(),
        viewmanager.statusline_view.clone(),
        &highlightengine
    )));

    let mut projecttree = Rc::new(RefCell::new(ProjectTree::new(
        messagemanager.get_messagesender(),
        viewmanager.left_view.clone(),
        &highlightengine
    )));

    texteditor.borrow_mut().display();
    statusline.borrow_mut().display();
    //projecttree.borrow_mut().display();

    let mut selectormanager = Rc::new(RefCell::new(SelectorManager::new(texteditor.clone())));

    messagemanager.register(texteditor);
    messagemanager.register(statusline);
    messagemanager.register(selectormanager);

    let terminalevent_watcher = TerminalEventWatcher::new(
        messagemanager.get_messagesender(),
    );
    terminalevent_watcher.start_watch_thread();

    messagemanager.start_loop(message_recv);

    { terminal.lock().unwrap().finish(); }

}
