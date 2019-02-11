use std::sync::mpsc;
use std::rc::Rc;
use std::cell::RefCell;
use super::Message;

pub trait MessageListener {
    fn on_message(&mut self, message: Message);
}

pub struct MessageManager {
    messagesender: mpsc::Sender<Message>,
    message_listeners: Vec<Rc<RefCell<MessageListener>>>,
}

impl MessageManager {
    pub fn new() -> (MessageManager, mpsc::Receiver<Message>) {
        let (send, recv) = mpsc::channel();
        let messagemanager = MessageManager {
            messagesender: send,
            message_listeners: Vec::new(),
        };
        (messagemanager, recv)
    }

    pub fn register(&mut self, messagelistener: Rc<RefCell<MessageListener>>) {
        self.message_listeners.push(messagelistener);
    }

    pub fn start_loop(&mut self, message_recv : mpsc::Receiver<Message>) {
        loop {
            if let Ok(message) = message_recv.recv() {
                if let Message::Quit = message {
                    break;
                };
                for message_listener in &self.message_listeners {
                    message_listener.borrow_mut().on_message(message.clone());
                }
            }
        }
    }

    pub fn get_messagesender(&self) -> mpsc::Sender<Message> {
        self.messagesender.clone()
    }
}
