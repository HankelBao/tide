use super::TextBuffer;

use std::io::BufReader;
use std::io::prelude::{*};
use std::fs::File;

pub trait FileRW {
    fn set_file_path(&mut self, file_name: String);
    fn load_file(&mut self);
    fn save_file(&self);
}

impl<'a> FileRW for TextBuffer<'a> {
    fn set_file_path(&mut self, file_name: String) {
        self.file_path = file_name;
    }

    fn load_file(&mut self) {
        let f = match File::open(self.file_path.clone()) {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let buffered_f = BufReader::new(f);

        self.lines.clear();
        for line in buffered_f.lines() {
            self.lines.push(Box::new(line.unwrap()));
        }
    }

    fn save_file(&self) {
        let mut f = match File::create(self.file_path.clone()) {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        f.write_all(self.as_string().as_bytes()).unwrap();
    }
}