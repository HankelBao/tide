use crate::ui::View;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct FuzzyFinder {
    view: Arc<Mutex<View>>,
    display_recv: Arc<Mutex<mpsc::Receiver<Vec<String>>>>,
}

impl FuzzyFinder {
}
