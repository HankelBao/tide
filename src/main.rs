extern crate termion;

mod terminal;
mod textbuffer;

use std::time::Duration;
use std::thread;

fn main() {
    /*let mut terminal = terminal::new();
    let mut content = vec![String::from("Test"), String::from("Another Test")];
    terminal.set_content(1, 1, 20, 10, content);
    terminal.flush();

    thread::sleep(Duration::from_millis(2000));
    terminal.finish();*/

    let mut textbuffer = textbuffer::TextBuffer::new();
    textbuffer.insert_at(0, 0, 'a');
    textbuffer.insert_at(1, 0, 'b');
    textbuffer.insert_at(2, 0, 'b');
    textbuffer.insert_at(3, 0, 'b');
    textbuffer.insert_at(4, 0, 'b');
    textbuffer.insert_at(1, 0, 'c');
    textbuffer.debug_print();
}
