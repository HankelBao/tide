use crate::ui::View;
use crate::terminal::DisplayLine;
use crate::core::HighlightEngine;
use crate::core::Style;
use crate::terminal::StyleDescriptor;
use crate::core::{Message, MessageSender, MessageListener};
use crate::ui::UIComponent;

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub struct Statusline {
    messagesender: MessageSender,
    file_name: String,
    line_num: String,
    line_offset: String,
    view: Arc<Mutex<View>>,
    style: Style,
}

impl Statusline {
    pub fn new(messagesender: MessageSender, view: Arc<Mutex<View>>, highlightengine: &HighlightEngine) -> Statusline {
        let statusline = Statusline {
            messagesender,
            file_name: String::from("nil"),
            line_num: String::from("nil"),
            line_offset: String::from("nil"),
            view,
            style: highlightengine.inversed_style.clone(),
        };
        statusline
    }
}

impl UIComponent for Statusline {
    fn display(&mut self) {
        let mut v = self.view.lock().unwrap();
        let width = v.get_width() as usize;
        let file_info = self.file_name.clone();
        let cursor_info = self.line_num.clone()+":"+&self.line_offset.clone();

        let left_aligned = file_info;
        let right_aligned = cursor_info;

        let mut display_content: String = " ".repeat(width);
        display_content.replace_range(..left_aligned.len(), &left_aligned);
        display_content.replace_range(width-right_aligned.len().., &right_aligned);
        let displayline = DisplayLine::from(display_content, vec![StyleDescriptor::from(self.style.clone(), 0)]);

        v.set_content(vec![displayline]);
        v.flush();
    }
}

impl MessageListener for Statusline {
    fn on_message(&mut self, message: Message) {
        match message {
            Message::FocusFileUpdate(file_name) => {
                self.file_name = file_name;
                self.display();
            },
            Message::FocusCursorMove(line_num, line_offset) => {
                self.line_num = line_num.to_string();
                self.line_offset = line_offset.to_string();
                self.display();
            },
            _ => {},
        }
    }
}
