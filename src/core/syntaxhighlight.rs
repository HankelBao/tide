use super::TextBuffer;

use syntect::easy::HighlightLines;
use syntect::parsing::{SyntaxSet, SyntaxReference};
use crate::terminal::{DisplayLine, StyleDescriptor};
use syntect::highlighting::{ThemeSet};

pub struct HighlightEngine {
    pub ps: SyntaxSet,
    pub ts: ThemeSet,
}

impl HighlightEngine {
    pub fn new() -> HighlightEngine {
        return HighlightEngine {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    pub fn get_syntax<'a>(&'a self, file_path: String) -> &'a SyntaxReference {
        return match self.ps.find_syntax_for_file(&file_path) {
            Ok(o) => {
                match o {
                    Some(s) => s,
                    None => self.get_syntax_plain_text(),
                }
            },
            Err(_) => self.get_syntax_plain_text()
        }
    }

    pub fn get_syntax_plain_text<'a>(&'a self) -> &'a SyntaxReference {
        return self.ps.find_syntax_plain_text();
    }
}

pub trait SyntaxHighlight<'a> {
    fn update_syntax(&mut self, highlightengine: &'a HighlightEngine);
    fn highlight(&mut self, highlightengine: &'a HighlightEngine);
}

impl<'a> SyntaxHighlight<'a> for TextBuffer<'a> {
    fn update_syntax(&mut self, highlightengine: &'a HighlightEngine) {
        self.syntax = highlightengine.get_syntax(self.file_path.clone());
        self.syntax_name = self.syntax.name.clone();
    }

    fn highlight(&mut self, highlightengine: &'a HighlightEngine) {
        let mut h = HighlightLines::new(self.syntax, &highlightengine.ts.themes["base16-ocean.dark"]);
        self.rendered_cache.clear();
        for line in &self.lines {
            let mut styles: Vec<StyleDescriptor> = Vec::new();
            for (style, sub_string) in h.highlight(line, &highlightengine.ps) {
                styles.push(StyleDescriptor::new(sub_string.len(), style));
            };
            self.rendered_cache.push(DisplayLine::new(*line.clone(), styles));
        }
    }
}
