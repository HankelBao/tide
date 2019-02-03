mod textbuffer;
mod textediting;
mod syntaxhighlight;
mod textdisplay;
mod filerw;

pub use self::textbuffer::TextBuffer;
pub use self::textediting::TextEditing;
pub use self::syntaxhighlight::HighlightEngine;
pub use self::syntaxhighlight::SyntaxHighlight;
pub use self::textdisplay::TextDisplay;
pub use self::filerw::FileRW;
pub use syntect::highlighting::{Style, FontStyle};
pub use syntect::parsing::SyntaxReference;