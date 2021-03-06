#[macro_use]
extern crate lazy_static;
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
use crate::core::GLog;
use crate::components::TextEditor;
use crate::components::Statusline;
use crate::components::ProjectTree;
use crate::ui::ViewManager;
use crate::ui::{UIComponent, UISelector};
use crate::ui::SelectorManager;
use crate::ui::TerminalEventWatcher;
use crate::ui::ComponentManager;
use crate::terminal::Terminal;

fn main() {
    { GLog.lock().unwrap().append("Log started...".to_string()); }

    let (mut messagemanager, message_recv) = MessageManager::new();
    let terminal = Rc::new(RefCell::new(Terminal::new()));
    let terminalevent_watcher = TerminalEventWatcher::new(
        messagemanager.get_messagesender(),
    );
    terminalevent_watcher.start_watch_thread();

    let highlightengine = HighlightEngine::new(Some(String::from("./themes")), String::from("OneDark"));
    { GLog.lock().unwrap().append("HighlightEngine initiated".to_string()); }

    let viewmanager = Rc::new(RefCell::new(ViewManager::new(messagemanager.get_messagesender(), terminal.clone())));
    let componentmanager = Rc::new(RefCell::new(ComponentManager::new()));
    let selectormanager = Rc::new(RefCell::new(SelectorManager::new()));
    messagemanager.register(viewmanager.clone());
    messagemanager.register(componentmanager.clone());
    messagemanager.register(selectormanager.clone());

    let args: Vec<String> = env::args().collect();

    let texteditor: Rc<RefCell<TextEditor>> = Rc::new(RefCell::new(match args.get(1) {
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

    selectormanager.borrow_mut().switch_focus(texteditor.clone());
    messagemanager.register(texteditor.clone());
    componentmanager.borrow_mut().register(texteditor.clone());

    let statusline: Rc<RefCell<Statusline>> = Rc::new(RefCell::new(Statusline::new(
        messagemanager.get_messagesender(),
        viewmanager.borrow().statusline_view.clone(),
        &highlightengine
    )));
    messagemanager.register(statusline.clone());
    componentmanager.borrow_mut().register(statusline.clone());

    let projecttree = Rc::new(RefCell::new(ProjectTree::new(
        messagemanager.get_messagesender(),
        viewmanager.borrow().left_view.clone(),
        &highlightengine
    )));
    componentmanager.borrow_mut().register(projecttree.clone());

    messagemanager.start_loop(message_recv);

    { GLog.lock().unwrap().append("Tide Exited".to_string()); }

    { GLog.lock().unwrap().save("./tide_log"); }

}
