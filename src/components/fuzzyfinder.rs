use crate::ui::View;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct FuzzyFinder {
    view: Arc<Mutex<View>>,
}

impl FuzzyFinder {
}
