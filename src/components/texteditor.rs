extern crate termion;
use termion::event::Key;

use crate::core::TextBuffer;
use crate::terminal::Terminal;
use crate::core::TextDisplay;
use crate::core::HighlightEngine;
use crate::core::TextEditing;
use crate::core::FileRW;
use crate::ui::View;

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub struct TextEditor {
    view: Arc<Mutex<View>>,

    textbuffers: Vec<Arc<Mutex<TextBuffer>>>,
    current_textbuffer: Arc<Mutex<TextBuffer>>,

    statusline_display_send: mpsc::Sender<(String, (u16, u16))>,
}

impl TextEditor {
    pub fn new(view: Arc<Mutex<View>>, highlightengine: &HighlightEngine, statusline_display_send: mpsc::Sender<(String, (u16, u16))>) -> TextEditor {
        let first_textbuffer = Arc::new(Mutex::new(TextBuffer::new(highlightengine)));
        let texteditor = TextEditor {
            view,
            textbuffers: vec![first_textbuffer.clone()],
            current_textbuffer: first_textbuffer.clone(),
            statusline_display_send,
        };
        return texteditor;
    }

    pub fn new_with_file(view: Arc<Mutex<View>>, highlightengine: &HighlightEngine, file_path: String, statusline_display_send: mpsc::Sender<(String, (u16, u16))>) -> TextEditor {
        let first_textbuffer = Arc::new(Mutex::new(TextBuffer::from_file(highlightengine, file_path)));
        let texteditor = TextEditor {
            view,
            textbuffers: vec![first_textbuffer.clone()],
            current_textbuffer: first_textbuffer.clone(),
            statusline_display_send,
        };
        return texteditor;
    }

    pub fn start_display_thread(&self) {
        let view = self.view.clone();
        let current_textbuffer = self.current_textbuffer.clone();
        thread::spawn(move || {
            loop {
                {
                    let mut textbuffer = current_textbuffer.lock().unwrap();
                    let display_recv_raw = textbuffer.display_recv.clone();
                    let display_recv = display_recv_raw.lock().unwrap();
                    if let Ok(_) = display_recv.try_recv() {
                        let mut v = view.lock().unwrap();
                        let (t_width, t_height) = v.get_scale();
                        textbuffer.adjust_viewpoint(t_width as u32, t_height as u32);
                        let display_lines = textbuffer.get_display_lines(t_width as u32, t_height as u32);
                        v.set_content(display_lines);

                        let (cursor_x, cursor_y) = textbuffer.get_local_cursor();
                        v.set_cursor(cursor_x, cursor_y);
                        v.flush();
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn key(&self, key: Key) {
        let mut t = self.current_textbuffer.lock().unwrap();
        match key {
            Key::Char(ch)   => t.insert(ch),
            Key::Ctrl('a')  => t.head(),
            Key::Ctrl('e')  => t.end(),
            Key::Ctrl('u')  => t.del_to_head(),
            Key::Ctrl('h')  => t.del_to_end(),
            Key::Ctrl('b')  => t.left(),
            Key::Ctrl('f')  => t.right(),
            Key::Ctrl('p')  => t.up(),
            Key::Ctrl('n')  => t.down(),
            Key::Ctrl('s')  => t.save_file(),
            Key::Backspace  => t.backspace(),
            Key::Up         => t.up(),
            Key::Down       => t.down(),
            Key::Left       => t.left(),
            Key::Right      => t.right(),
            _ => {},
        }
        self.statusline_display_send.send((t.file_path.clone(), (t.line_offset as u16, t.line_num as u16))).unwrap();
        { t.display_send.lock().unwrap().send(true).unwrap(); }
    }
}
