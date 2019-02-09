extern crate termion;
use termion::event::Key;

use crate::core::{Message, MessageListener};
use crate::core::TextBuffer;
use crate::terminal::Terminal;
use crate::core::TextDisplay;
use crate::core::HighlightEngine;
use crate::core::SyntaxHighlight;
use crate::core::TextEditing;
use crate::core::FileRW;
use crate::ui::View;
use crate::ui::{UIComponent, UISelector};

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub struct TextEditor {
    messagesender: mpsc::Sender<Message>,
    view: Arc<Mutex<View>>,

    textbuffers: Vec<TextBuffer>,
    current_textbuffer_index: usize,
}

impl TextEditor {
    pub fn new(messagesender: mpsc::Sender<Message>, view: Arc<Mutex<View>>, highlightengine: &HighlightEngine) -> TextEditor {
        let mut first_textbuffer = TextBuffer::new(0, messagesender.clone());
        first_textbuffer.start_highlight_thread(highlightengine);
        first_textbuffer.view_height = {view.lock().unwrap().get_height() as u32};
        first_textbuffer.highlight_from(0);
        let texteditor = TextEditor {
            messagesender,
            view,
            textbuffers: vec![first_textbuffer],
            current_textbuffer_index: 0,
        };
        return texteditor;
    }

    pub fn new_with_file(messagesender: mpsc::Sender<Message>, view: Arc<Mutex<View>>, highlightengine: &HighlightEngine, file_path: String) -> TextEditor {
        let mut first_textbuffer = TextBuffer::from_file(0, messagesender.clone(), file_path.clone());
        first_textbuffer.start_highlight_thread(highlightengine);
        first_textbuffer.view_height = {view.lock().unwrap().get_height() as u32};
        first_textbuffer.highlight_from(0);
        messagesender.send(Message::FocusFileUpdate(file_path.clone())).unwrap();
        let texteditor = TextEditor {
            messagesender,
            view,
            textbuffers: vec![first_textbuffer],
            current_textbuffer_index: 0,
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
        let mut textbuffer = &mut self.textbuffers[self.current_textbuffer_index];
        let mut v = self.view.lock().unwrap();
        let (t_width, t_height) = v.get_scale();
        textbuffer.adjust_viewpoint(t_width as u32, t_height as u32);
        let display_lines = textbuffer.get_display_lines(t_width as u32, t_height as u32);
        v.set_content(display_lines);

        let (cursor_x, cursor_y) = textbuffer.get_local_cursor();
        v.set_cursor(cursor_x, cursor_y);
        v.flush();
    }
}

impl UISelector for TextEditor {
    fn key(&mut self, key: Key) {
        match key {
            Key::Char(ch)   => self.textbuffers[self.current_textbuffer_index].insert(ch),
            Key::Ctrl('a')  => self.textbuffers[self.current_textbuffer_index].head(),
            Key::Ctrl('e')  => self.textbuffers[self.current_textbuffer_index].end(),
            Key::Ctrl('u')  => self.textbuffers[self.current_textbuffer_index].del_to_head(),
            Key::Ctrl('h')  => self.textbuffers[self.current_textbuffer_index].del_to_end(),
            Key::Ctrl('b')  => self.textbuffers[self.current_textbuffer_index].left(),
            Key::Ctrl('f')  => self.textbuffers[self.current_textbuffer_index].right(),
            Key::Ctrl('p')  => self.textbuffers[self.current_textbuffer_index].up(),
            Key::Ctrl('n')  => self.textbuffers[self.current_textbuffer_index].down(),
            Key::Ctrl('s')  => self.textbuffers[self.current_textbuffer_index].save_file(),
            Key::Backspace  => self.textbuffers[self.current_textbuffer_index].backspace(),
            Key::Up         => self.textbuffers[self.current_textbuffer_index].up(),
            Key::Down       => self.textbuffers[self.current_textbuffer_index].down(),
            Key::Left       => self.textbuffers[self.current_textbuffer_index].left(),
            Key::Right      => self.textbuffers[self.current_textbuffer_index].right(),
            _ => {},
        }
        self.messagesender.send(Message::FocusCursorMove(
            self.textbuffers[self.current_textbuffer_index].line_num as u16,
            self.textbuffers[self.current_textbuffer_index].line_offset as u16,
        )).unwrap();
        self.display();
    }
}
