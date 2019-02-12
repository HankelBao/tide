extern crate termion;
use termion::event::Key;

use crate::core::{Message, MessageListener};
use crate::core::TextBuffer;
use crate::core::TextDisplay;
use crate::core::HighlightEngine;
use crate::core::SyntaxHighlight;
use crate::core::TextEditing;
use crate::core::FileRW;
use crate::ui::View;
use crate::ui::{UIComponent, UISelector};
use crate::core::Style;

use std::sync::mpsc;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TextEditor {
    messagesender: mpsc::Sender<Message>,
    view: Rc<RefCell<View>>,

    textbuffers: Vec<TextBuffer>,
    current_textbuffer_index: usize,
    default_style: Style,
}

impl TextEditor {
    pub fn new(messagesender: mpsc::Sender<Message>, view: Rc<RefCell<View>>, highlightengine: &HighlightEngine) -> TextEditor {
        let mut first_textbuffer = TextBuffer::new(0, messagesender.clone(), highlightengine);
        first_textbuffer.start_highlight_thread(highlightengine);
        first_textbuffer.view_height = 200;
        first_textbuffer.highlight_from(0);
        messagesender.send(Message::FocusCursorMove(0, 0)).unwrap();
        let texteditor = TextEditor {
            messagesender,
            view: view,
            textbuffers: vec![first_textbuffer],
            current_textbuffer_index: 0,
            default_style: highlightengine.default_style.clone(),
        };
        return texteditor;
    }

    pub fn new_with_file(messagesender: mpsc::Sender<Message>, view: Rc<RefCell<View>>, highlightengine: &HighlightEngine, file_path: String) -> TextEditor {
        let mut first_textbuffer = TextBuffer::from_file(0, messagesender.clone(), highlightengine, file_path.clone());
        first_textbuffer.start_highlight_thread(highlightengine);
        first_textbuffer.view_height = 200;
        first_textbuffer.highlight_from(0);
        messagesender.send(Message::FocusFileUpdate(file_path.clone())).unwrap();
        messagesender.send(Message::FocusCursorMove(0, 0)).unwrap();
        messagesender.send(Message::FocusSyntaxUpdate(first_textbuffer.syntax_name.clone())).unwrap();
        let texteditor = TextEditor {
            messagesender,
            view,
            textbuffers: vec![first_textbuffer],
            current_textbuffer_index: 0,
            default_style: highlightengine.default_style.clone(),
        };
        return texteditor;
    }

}

impl MessageListener for TextEditor {
    fn on_message(&mut self, message: Message) {
        match message {
            Message::HighlightReady(buffer_index) => {
                if buffer_index == self.current_textbuffer_index {
                    self.display();
                }
            },
            Message::TerminalKey(key) => {
            }
            _ => {},
        }
    }
}

impl UIComponent for TextEditor {
    fn display(&mut self) {
        let textbuffer = &mut self.textbuffers[self.current_textbuffer_index];
        let v = self.view.borrow();
        let (t_width, t_height) = v.get_scale();
        textbuffer.adjust_viewpoint(t_width as u32, t_height as u32);
        let display_lines = textbuffer.get_display_lines(t_width as u32, t_height as u32);
        v.set_content(display_lines, self.default_style);
        v.flush();
    }
}

impl UISelector for TextEditor {
    fn display_cursor(&mut self) {
        let textbuffer = &mut self.textbuffers[self.current_textbuffer_index];
        let v = self.view.borrow();
        let (cursor_x, cursor_y) = textbuffer.get_local_cursor();
        v.set_cursor(cursor_x, cursor_y);
        v.flush();
    }

    fn key(&mut self, key: Key) {
        { // Scope where current textbuffer is edited.
            let textbuffer = &mut self.textbuffers[self.current_textbuffer_index];
            match key {
                Key::Char(ch)   => textbuffer.insert(ch),
                Key::Ctrl('a')  => textbuffer.head(),
                Key::Ctrl('e')  => textbuffer.end(),
                Key::Ctrl('u')  => textbuffer.del_to_head(),
                Key::Ctrl('h')  => textbuffer.del_to_end(),
                Key::Ctrl('b')  => textbuffer.left(),
                Key::Ctrl('f')  => textbuffer.right(),
                Key::Ctrl('p')  => textbuffer.up(),
                Key::Ctrl('n')  => textbuffer.down(),
                Key::Ctrl('s')  => textbuffer.save_file(),
                Key::Backspace  => textbuffer.backspace(),
                Key::Up         => textbuffer.up(),
                Key::Down       => textbuffer.down(),
                Key::Left       => textbuffer.left(),
                Key::Right      => textbuffer.right(),
                _ => {},
            }
            self.messagesender.send(Message::FocusCursorMove(
                textbuffer.line_num as u16,
                textbuffer.line_offset as u16,
            )).unwrap();
        }
        self.display();
    }
}