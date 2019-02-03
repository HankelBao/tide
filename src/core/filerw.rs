use super::TextBuffer;

use std::io::BufReader;
use std::io::prelude::{*};
use std::fs::File;

pub trait FileRW {
    fn set_file_path(&mut self, file_name: String);
    fn load_file(&mut self);
    fn save_file(&mut self);
}

impl<'a> FileRW for TextBuffer<'a> {
    fn set_file_path(&mut self, file_name: String) {
        self.file_path = file_name;
    }

    fn load_file(&mut self) {
        let file_name = self.file_path.clone();
        let f = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let buffered_f = BufReader::new(f);

        self.lines.clear();
        for line in buffered_f.lines() {
            self.lines.push(Box::new(line.unwrap()));
        }
    }

    fn save_file(&mut self) {
        // Not implemented yet.
    }
}