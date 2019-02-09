use crate::terminal::Key;

pub trait UISelector {
    fn key(&mut self, key: Key);
}
