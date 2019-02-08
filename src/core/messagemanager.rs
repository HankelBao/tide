use std::sync::mpsc;
use std::collections::HashMap;
use super::Message;
use super::MessageKey;

pub trait MessageListener<T> {
    fn on_message(&self, message: Message<T>);
}

pub struct MessageManager<'a, T> {
    event_send: mpsc::Sender<Message<T>>,
    message_listeners: HashMap<MessageKey, Vec<&'a MessageListener<T>>>,
}

impl<'a, T> MessageManager<'a, T> {
    fn new() -> (MessageManager<'a, T>, mpsc::Receiver<Message<T>>) {
        let (send, recv) = mpsc::channel();
        let messagemanager = MessageManager {
            event_send: send,
            message_listeners: HashMap::new(),
        };
        (messagemanager, recv)
    }

    fn register(&mut self, messagekey: MessageKey, messagelistener: &'a MessageListener<T>) {
        let message_listeners = self.message_listeners.entry(messagekey).or_insert(Vec::new());
        message_listeners.push(messagelistener);
    }
}
