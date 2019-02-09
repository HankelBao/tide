use crate::core::{Message, MessageSender};
use crate::ui::{View};

use std::sync::{Arc, Mutex};

pub struct ProjectTree {
    messagesender: MessageSender,
    view: Arc<Mutex<View>>,
}
