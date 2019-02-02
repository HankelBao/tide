use super::TextBuffer;

use syntect::easy::HighlightLines;
use syntect::parsing::{SyntaxSet};
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
}

pub trait SyntaxHighlight {
    fn highlight(&mut self, ps: &SyntaxSet, ts: &ThemeSet);
}

impl SyntaxHighlight for TextBuffer {
    fn highlight(&mut self, ps: &SyntaxSet, ts: &ThemeSet) {
        let syntax = ps.find_syntax_by_extension("rs").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        self.rendered_cache.clear();
        for line in &self.lines {
            let mut styles: Vec<StyleDescriptor> = Vec::new();
            for (style, sub_string) in h.highlight(line, ps) {
                styles.push(StyleDescriptor::new(sub_string.len(), style));
            };
            self.rendered_cache.push(DisplayLine::new(*line.clone(), styles));
        }
    }
}
