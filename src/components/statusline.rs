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
use std::rc::Rc;
use std::cell::RefCell;

pub struct Statusline {
    messagesender: MessageSender,
    file_name: String,
    syntax_name: String,
    line_num: String,
    line_offset: String,
    view: Rc<RefCell<View>>,
    style: Style,
}

impl Statusline {
    pub fn new(messagesender: MessageSender, view: Rc<RefCell<View>>, highlightengine: &HighlightEngine) -> Statusline {
        let statusline = Statusline {
            messagesender,
            file_name: String::from("nil"),
            syntax_name: String::from("Non-highlight"),
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
        let mut v = self.view.borrow_mut();
        let width = v.get_width() as usize;
        let file_info = self.file_name.clone();
        let cursor_info = String::from("Line ") + &self.line_num.clone() + ", Column " + &self.line_offset.clone();
        let syntax_name = self.syntax_name.clone();

        let left_aligned = String::from("» ") + &file_info + ", " + &cursor_info;
        let right_aligned = syntax_name;

        let mut display_content: String = " ".repeat(width);
        if width < left_aligned.len() + right_aligned.len() {
            display_content.clear();
        } else {
            display_content.replace_range(..left_aligned.len(), &left_aligned);
            display_content.replace_range(width-right_aligned.len().., &right_aligned);
        }
        let displayline = DisplayLine::from(display_content, vec![StyleDescriptor::from(self.style.clone(), 0)]);

        v.set_content(vec![displayline], self.style);
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
                self.line_num = (line_num+1).to_string();
                self.line_offset = (line_offset+1).to_string();
                self.display();
            },
            Message::FocusSyntaxUpdate(syntax_name) => {
                self.syntax_name = syntax_name;
                self.display();
            }
            _ => {},
        }
    }
}
