
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::highlighting::{Style, Color, FontStyle};
use std::path::Path;

pub struct HighlightEngine {
    pub ps: SyntaxSet,
    ts: ThemeSet,
    pub theme: Theme,
    pub default_style: Style,
    pub inversed_style: Style,
    pub line_num_style: Style,
    pub projecttree_style: Style,
}

impl<'a> HighlightEngine {
    pub fn new(theme_folder: Option<String>, theme_name: String) -> HighlightEngine {
        let ps = SyntaxSet::load_defaults_nonewlines();
        let ts: ThemeSet;
        if let Some(theme_folder_name) = theme_folder {
            ts = ThemeSet::load_from_folder(Path::new(&theme_folder_name)).unwrap();
        } else {
            ts = ThemeSet::load_defaults();
        }
        let theme = ts.themes[&theme_name].clone();
        HighlightEngine {
            ps,
            ts,
            theme: theme.clone(),
            default_style: Style {
                background: theme.settings.background.unwrap_or_else(|| Color::BLACK),
                foreground: theme.settings.foreground.unwrap_or_else(|| Color::WHITE),
                font_style: FontStyle::empty(),
            },
            inversed_style: Style {
                background: theme.settings.foreground.unwrap_or_else(|| Color::WHITE),
                foreground: theme.settings.background.unwrap_or_else(|| Color::BLACK),
                font_style: FontStyle::empty(),
            },
            line_num_style: Style {
                foreground: theme.settings.foreground.unwrap_or_else(|| Color::WHITE),
                background: theme.settings.gutter.unwrap_or_else(|| Color::BLACK),
                font_style: FontStyle::empty(),
            },
            projecttree_style: Style {
                foreground: theme.settings.foreground.unwrap_or_else(|| Color::WHITE),
                background: theme.settings.gutter.unwrap_or_else(|| Color::BLACK),
                font_style: FontStyle::empty(),
            }
        }
    }
}
