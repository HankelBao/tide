
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::highlighting::{Style, Color, FontStyle};

pub struct HighlightEngine {
    pub ps: SyntaxSet,
    pub ts: ThemeSet,
    pub default_style: Style,
    pub inversed_style: Style,
}

impl<'a> HighlightEngine {
    pub fn new() -> HighlightEngine {
        let ps = SyntaxSet::load_defaults_nonewlines();
        let ts = ThemeSet::load_defaults();
        HighlightEngine {
            ps,
            ts,
            default_style: Style {
                background: Color::BLACK,
                foreground: Color::WHITE,
                font_style: FontStyle::empty(),
            },
            inversed_style: Style {
                background: Color::WHITE,
                foreground: Color::BLACK,
                font_style: FontStyle::empty(),
            },
        }
    }
}
