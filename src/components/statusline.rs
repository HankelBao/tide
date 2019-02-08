use crate::ui::View;
use crate::terminal::DisplayLine;
use crate::core::HighlightEngine;
use crate::core::Style;
use crate::terminal::StyleDescriptor;

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub struct Statusline {
    view: Arc<Mutex<View>>,
    style: Style,
    display_recv: Arc<Mutex<mpsc::Receiver<(String, (u16, u16))>>>,
}

impl Statusline {
    pub fn new(view: Arc<Mutex<View>>, highlightengine: &HighlightEngine, display_recv: mpsc::Receiver<(String, (u16, u16))>) -> Statusline {
        let statusline = Statusline {
            view,
            style: highlightengine.inversed_style.clone(),
            display_recv: Arc::new(Mutex::new(display_recv)),
        };
        statusline
    }

    pub fn start_display_thread(&self) {
        let view = self.view.clone();
        let display_recv = self.display_recv.clone();
        let display_style = self.style.clone();
        thread::spawn(move || {
            loop {
                {
                    if let Ok((file_name, (line_offset, line_num))) = display_recv.lock().unwrap().try_recv() {
                        let mut v = view.lock().unwrap();
                        let width = v.get_width() as usize;
                        let file_info = file_name;
                        let cursor_info = line_num.to_string()+":"+&line_offset.to_string();

                        let left_aligned = file_info;
                        let right_aligned = cursor_info;

                        let mut display_content: String = " ".repeat(width);
                        display_content.replace_range(..left_aligned.len(), &left_aligned);
                        display_content.replace_range(width-right_aligned.len().., &right_aligned);
                        let displayline = DisplayLine::from(display_content, vec![StyleDescriptor::from(display_style.clone(), 0)]);
                        v.set_content(vec![displayline]);
                        v.flush();
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
}
