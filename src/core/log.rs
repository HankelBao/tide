extern crate lazy_static;

use std::io::prelude::*;
use std::fs::File;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref GLog: Arc<Mutex<Log>> = {
        Arc::new(Mutex::new(Log::new()))
    };
}

use std::time::Duration;
use std::time::Instant;

struct LogItem {
    elapsed: Duration,
    info: String,
}

impl LogItem {
    fn to_string(&self) -> String {
        format!("[{:?}] {}", self.elapsed, self.info)
    }
}

pub struct Log {
    start: Instant,
    items: Vec<LogItem>,
}

impl Log {
    pub fn new() -> Log {
        let mut log = Log {
            start: Instant::now(),
            items: Vec::new(),
        };
        log
    }

    pub fn append(&mut self, info: String) {
        let item = LogItem {
            elapsed: self.start.elapsed(),
            info,
        };
        self.items.push(item);
    }

    pub fn print(&self) {
        for item in self.items.iter() {
            println!("{}", item.to_string());
        }
    }

    pub fn save(&self, file_path: &str) {
        let mut f = match File::create(file_path.clone()) {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        for item in self.items.iter() {
            f.write(item.to_string().as_bytes()).unwrap();
            f.write("\n".as_bytes()).unwrap();
        }
        f.flush().unwrap();
    }
}
