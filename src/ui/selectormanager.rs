use super::UISelector;
use crate::core::{MessageListener, Message};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SelectorManager {
    focus_selector: Rc<RefCell<UISelector>>,
}

impl SelectorManager {
    pub fn new(uiselector: Rc<RefCell<UISelector>>) -> SelectorManager {
        SelectorManager {
            focus_selector: uiselector,
        }
    }
}

impl MessageListener for SelectorManager {
    fn on_message(&mut self, message: Message) {
        match message {
            Message::TerminalKey(key) => {
                self.focus_selector.borrow_mut().key(key);
                self.focus_selector.borrow_mut().display_cursor();
            },
            _ => {},
        }
    }
}
