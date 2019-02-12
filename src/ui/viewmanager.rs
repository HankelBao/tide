use crate::terminal::Terminal;
use super::View;
use crate::core::{Message, MessageListener, MessageSender};
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc;

/*
 * TODO:
 * add a resize detector and rewrite view.
 */

pub struct ViewManager {
    messagesender: MessageSender,
    terminal: Rc<RefCell<Terminal>>,
    /*
     * All the views will be automatically adjusted
     * by a thread.
     */
    pub left_view: Rc<RefCell<View>>,
    pub right_view: Rc<RefCell<View>>,
    pub bottom_view: Rc<RefCell<View>>,
    pub statusline_view: Rc<RefCell<View>>,
    /*
     * Main View will automatically adjusted
     * by other views by the thread.
     */
    pub main_view: Rc<RefCell<View>>,

    /*
     * Cache the scale for resize of views.
     */
    width: u16,
    height: u16,
}

impl ViewManager {
    pub fn new(messagesender: MessageSender, terminal: Rc<RefCell<Terminal>>) -> ViewManager {
        /*
         * It would be adjusted by the initial
         * TerminalResize signal.
         */
        let (t_width, t_height) = (0, 0);
        let null_view = View::new(terminal.clone());
        let vm = ViewManager {
            messagesender,
            terminal: terminal.clone(),
            left_view:      Rc::new(RefCell::new(null_view.clone())),
            right_view:     Rc::new(RefCell::new(null_view.clone())),
            bottom_view:    Rc::new(RefCell::new(null_view.clone())),
            statusline_view:Rc::new(RefCell::new(null_view.clone())),
            main_view:      Rc::new(RefCell::new(null_view.clone())),
            width: 0,
            height: 0,
        };
        vm
    }

    pub fn adjust_views(&mut self) {
        /*
         * adjust heights
         * bottom_view don't need to update.
         */
        self.left_view.borrow_mut().height = self.height - 1;
        self.right_view.borrow_mut().height = self.height - 1;
        self.statusline_view.borrow_mut().height = 1;
        self.main_view.borrow_mut().height = self.height - self.bottom_view.borrow().height - 1;

        /*
         * adjust widths
         * left_view, right_view don't need to update.
         */
        self.statusline_view.borrow_mut().width = self.width+1;
        self.main_view.borrow_mut().width = self.width - self.left_view.borrow().width - self.right_view.borrow().width;
        self.bottom_view.borrow_mut().width = self.width - self.left_view.borrow().width - self.right_view.borrow().width;

        /*
         * adjust start_x
         */
        self.left_view.borrow_mut().start_x = 1;
        self.statusline_view.borrow_mut().start_x = 1;
        self.main_view.borrow_mut().start_x = self.left_view.borrow().width+1;
        self.bottom_view.borrow_mut().start_x = self.left_view.borrow().width+1;
        self.right_view.borrow_mut().start_x = self.left_view.borrow().width + self.main_view.borrow().width + 1;

        /*
         * adjust start_y
         */
        self.left_view.borrow_mut().start_y = 1;
        self.main_view.borrow_mut().start_y = 1;
        self.right_view.borrow_mut().start_y = 1;
        self.bottom_view.borrow_mut().start_y = self.main_view.borrow().height+1;
        self.statusline_view.borrow_mut().start_y = self.height;
    }
}

impl MessageListener for ViewManager {
    fn on_message(&mut self, message: Message) {
        match message {
            Message::TerminalResize(width, height) => {
                self.terminal.borrow_mut().clear();
                self.width = width;
                self.height = height;
                self.adjust_views();
                self.messagesender.send(Message::RedrawAllUIComponents).unwrap();
            },
            _ => {},
        }
    }
}
