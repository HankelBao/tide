use super::UIComponent;
use crate::core::{Message, MessageListener};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ComponentManager {
    components: Vec<Rc<RefCell<UIComponent>>>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            components: Vec::new(),
        }
    }

    pub fn register(&mut self, uicomponent: Rc<RefCell<UIComponent>>) {
        self.components.push(uicomponent);
    }

    pub fn refresh_all(&self) {
        for component in &self.components {
            component.borrow_mut().display();
        }
    }
}

impl MessageListener for ComponentManager {
    fn on_message(&mut self, message: Message) {
        if let Message::RedrawAllUIComponents = message {
            self.refresh_all();
        }
    }
}
