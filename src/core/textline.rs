use crate::terminal::DisplayLine;

#[derive(Debug)]
pub struct TextLine {
    pub content: String,
    pub cache: DisplayLine,
}

impl TextLine {
    pub fn new() -> TextLine {
        return TextLine {
            content: String::new(),
            cache: DisplayLine::new(),
        }
    }

    pub fn from(content: String) -> TextLine {
        return TextLine {
            content,
            cache: DisplayLine::new(),
        }
    }

    pub fn content(&self) -> String {
        return self.content.clone();
    }

    pub fn insert(&mut self, index: usize, ch: char) {
        self.content.insert(index, ch);
    }

    pub fn remove(&mut self, index: usize) {
        self.content.remove(index);
    }

    pub fn split_off(&mut self, index: usize) -> String {
        return self.content.split_off(index);
    }

    pub fn char_at(&mut self, index: usize) -> char {
        return self.content.chars().nth(index).unwrap();
    }

    pub fn len(&mut self) -> usize {
        return self.content.len();
    }

    pub fn push_str(&mut self, add_string: String) {
        self.content.push_str(&add_string);
    }
}