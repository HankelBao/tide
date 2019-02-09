use crate::terminal::Terminal;
use super::View;
use crate::core::{Message, MessageListener};
use std::sync::{Arc, Mutex};
use std::thread;

/*
 * TODO:
 * add a resize detector and rewrite view.
 */

pub struct ViewManager {
    terminal: Arc<Mutex<Terminal>>,
    /*
     * All the views will be automatically adjusted
     * by a thread.
     */
    pub left_view: Arc<Mutex<View>>,
    pub right_view: Arc<Mutex<View>>,
    pub bottom_view: Arc<Mutex<View>>,
    pub statusline_view: Arc<Mutex<View>>,
    /*
     * Main View will automatically adjusted
     * by other views by the thread.
     */
    pub main_view: Arc<Mutex<View>>,
}

impl ViewManager {
    pub fn new(terminal: Arc<Mutex<Terminal>>) -> ViewManager {
        let (t_width, t_height) =  { terminal.lock().unwrap().get_scale().clone() };
        let vm = ViewManager {
            terminal: terminal.clone(),
            left_view: Arc::new(Mutex::new(View::from(terminal.clone(), 1, 1, 0, t_height-1))),
            right_view: Arc::new(Mutex::new(View::from(terminal.clone(), t_width, 1, 0, t_height-1))),
            bottom_view: Arc::new(Mutex::new(View::from(terminal.clone(), 20, t_height, t_width, 0))),
            main_view: Arc::new(Mutex::new(View::from(terminal.clone(), 1, 1, t_width, t_height-1))),
            statusline_view: Arc::new(Mutex::new(View::from(terminal.clone(), 1, t_height, t_width, 1))),
        };
        vm
    }
}

impl MessageListener for ViewManager {
    fn on_message(&mut self, message: Message) {

    }
}
