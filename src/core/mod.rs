mod textline;
mod textbuffer;
mod textediting;
mod highlightengine;
mod syntaxhighlight;
mod textdisplay;
mod filerw;

pub use self::textbuffer::TextBuffer;
pub use self::textline::TextLine;
pub use self::textediting::TextEditing;
pub use self::syntaxhighlight::SyntaxHighlight;
pub use self::textdisplay::TextDisplay;
pub use self::filerw::FileRW;

pub use self::highlightengine::HighlightEngine;

pub use syntect::highlighting::{Style, FontStyle};
pub use syntect::parsing::SyntaxReference;