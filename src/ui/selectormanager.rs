use super::UISelector;
use crate::core::{MessageListener, Message};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SelectorManager {
    focus_selector: Option<Rc<RefCell<UISelector>>>,
}

impl SelectorManager {
    pub fn new() -> SelectorManager {
        SelectorManager {
            focus_selector: None,
        }
    }

    pub fn switch_focus(&mut self, uiselector: Rc<RefCell<UISelector>>) {
        self.focus_selector = Some(uiselector);
    }
}

impl MessageListener for SelectorManager {
    fn on_message(&mut self, message: Message) {
        match message {
            Message::TerminalKey(key) => {
                if let Some(uiselector) = &self.focus_selector {
                    uiselector.borrow_mut().key(key);
                    uiselector.borrow_mut().display_cursor();
                }
            },
            Message::RedrawAllUIComponents => {
                if let Some(uiselector) = &self.focus_selector {
                    uiselector.borrow_mut().display_cursor();
                }
            }
            _ => {},
        }
    }
}
