
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

pub struct HighlightEngine {
    pub ps: SyntaxSet,
    pub ts: ThemeSet,
}

impl<'a> HighlightEngine {
    pub fn new() -> HighlightEngine {
        let ps = SyntaxSet::load_defaults_nonewlines();
        let ts = ThemeSet::load_defaults();
        HighlightEngine {
            ps,
            ts,
        }
    }
}