use crate::terminal::Key;

pub trait UISelector {
    fn display_cursor(&mut self);
    fn key(&mut self, key: Key);
}
