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
use crate::ui::ComponentManager;
use crate::terminal::Terminal;

use termion::event::{Key, Event};

fn main() {
    let highlightengine = HighlightEngine::new(String::from("Solarized (dark)"));

    let (mut messagemanager, message_recv) = MessageManager::new();

    let terminal = Rc::new(RefCell::new(Terminal::new()));
    let viewmanager = Rc::new(RefCell::new(ViewManager::new(messagemanager.get_messagesender(), terminal.clone())));
    let componentmanager = Rc::new(RefCell::new(ComponentManager::new()));
    let selectormanager = Rc::new(RefCell::new(SelectorManager::new()));
    
    let terminalevent_watcher = TerminalEventWatcher::new(
        messagemanager.get_messagesender(),
    );
    terminalevent_watcher.start_watch_thread();

    let args: Vec<String> = env::args().collect();

    let mut texteditor: Rc<RefCell<TextEditor>> = Rc::new(RefCell::new(match args.get(1) {
        Some(path) => TextEditor::new_with_file(
            messagemanager.get_messagesender(),
            viewmanager.borrow().main_view.clone(),
            &highlightengine,
            path.clone()),
        None => TextEditor::new(
            messagemanager.get_messagesender(),
            viewmanager.borrow().main_view.clone(),
            &highlightengine),
    }));

    let mut statusline: Rc<RefCell<Statusline>> = Rc::new(RefCell::new(Statusline::new(
        messagemanager.get_messagesender(),
        viewmanager.borrow().statusline_view.clone(),
        &highlightengine
    )));

    let mut projecttree = Rc::new(RefCell::new(ProjectTree::new(
        messagemanager.get_messagesender(),
        viewmanager.borrow().left_view.clone(),
        &highlightengine
    )));

    selectormanager.borrow_mut().switch_focus(texteditor.clone());

    messagemanager.register(texteditor.clone());
    messagemanager.register(statusline.clone());
    messagemanager.register(viewmanager.clone());
    messagemanager.register(componentmanager.clone());
    messagemanager.register(selectormanager.clone());

    componentmanager.borrow_mut().register(texteditor.clone());
    componentmanager.borrow_mut().register(statusline.clone());


    messagemanager.start_loop(message_recv);

    terminal.borrow_mut().finish();

}
