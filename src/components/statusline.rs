use crate::ui::View;

use std::sync::mpsc;

pub struct Statusline {
    display_recv: mpsc::Receiver<(String, (u16, u16))>,
}

impl Statusline {
    pub fn new(display_recv: mpsc::Receiver<(String, (u16, u16))>) {
        let statusline = Statusline {
            display_recv,
        }
        statusline
    }

    pub fn start_display_thread(&self) {

    }
}