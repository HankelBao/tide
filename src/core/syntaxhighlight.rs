use crate::terminal::{DisplayLine};
use super::TextBuffer;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use syntect::parsing::{ParseState, SyntaxSet, SyntaxReference, ScopeStack};
use syntect::highlighting::{ThemeSet, Highlighter, HighlightState, HighlightIterator, Style};

pub struct HighlightEngine {
    pub ps: SyntaxSet,
    pub ts: ThemeSet,
}

impl<'a>  HighlightEngine {
    pub fn new() -> HighlightEngine {
        let ps = SyntaxSet::load_defaults_nonewlines();
        let ts = ThemeSet::load_defaults();
        HighlightEngine {
            ps,
            ts,
        }
    }

    pub fn get_syntax(&'a self, file_path: String) -> &'a SyntaxReference {
        match self.ps.find_syntax_for_file(&file_path) {
            Ok(o) => {
                match o {
                    Some(s) => s,
                    None => self.ps.find_syntax_plain_text()
                }
            },
            Err(_) => self.ps.find_syntax_plain_text()
        }
    }

    pub fn start_highlight(&'a self, textbuffer: &TextBuffer) -> mpsc::Sender<u32> {
        let lines = textbuffer.lines.clone();
        let file_path = textbuffer.file_path.clone();
        let ps = self.ps.clone();
        let theme = self.ts.themes["base16-ocean.dark"].clone();
        let (send, recv): (mpsc::Sender<u32>, mpsc::Receiver<u32>)= mpsc::channel();

        thread::spawn(move || {
            let syntax = match ps.find_syntax_for_file(&file_path) {
                Ok(o) => {
                    match o {
                        Some(s) => s,
                        None => ps.find_syntax_plain_text()
                    }
                },
                Err(_) => ps.find_syntax_plain_text()
            };
            let highlighter = Highlighter::new(&theme);
            let initial_highlight_state = HighlightState::new(&highlighter, ScopeStack::new());
            let initial_parse_state = ParseState::new(syntax);

            let mut current_line_num: usize = 0;
            let mut current_state = HighlightCache {
                highlight_state: initial_highlight_state,
                parse_state: initial_parse_state,
            };
            let mut state_cache = vec![current_state.clone()];


            loop {
                /*
                 * Check Refresh Signal
                 */
                if let Ok(request_line_num) = recv.try_recv() {
                    if (request_line_num as usize) < current_line_num {
                        /*
                         * Update cureent_line_num and current_state here.
                         */
                        let mut request_cache_point = (request_line_num / 100) as usize;
                        if request_cache_point >= state_cache.len() {
                            request_cache_point = state_cache.len() - 1;
                        }
                        current_line_num = request_cache_point * 100;
                        current_state = state_cache[request_cache_point].clone();
                    }
                }

                /*
                 * Check if necessary of render anything
                 */
                let lines_len = { lines.lock().unwrap().len().clone() };
                if current_line_num >= lines_len {
                    thread::sleep(Duration::from_millis(100));
                    continue
                } 

                /*
                 * Record when at cache point
                 */
                let cache_point = (current_line_num / 100) as usize;
                let cache_offset = current_line_num - (cache_point * 100);
                if cache_offset == 0 {
                    if cache_point < state_cache.len() {
                        state_cache[cache_point] = current_state.clone();
                    }
                    if cache_point == state_cache.len() {
                        state_cache.push(current_state.clone());
                    }
                }

                /*
                 * Render the current line
                 */
                let line_content: String = { lines.lock().unwrap()[current_line_num].content() };
                let ops = current_state.parse_state.parse_line(&line_content, &ps);
                let range_iter = HighlightIterator::new(&mut current_state.highlight_state, &ops[..], &line_content, &highlighter);
                let ranges: Vec<(Style, &str)> = range_iter.collect();

                { lines.lock().unwrap()[current_line_num].cache = DisplayLine::from(line_content.clone(), ranges); }

                
                current_line_num += 1;
            }
        });
        return send;
    }
}

#[derive(Clone)]
pub struct HighlightCache {
    highlight_state: HighlightState,
    parse_state: ParseState,
}

pub trait SyntaxHighlight {
/*    fn start_highlight_thread(&mut self);
    fn highlight_from(&self, index: u32);*/
}

/*impl SyntaxHighlight for TextBuffer {
}*/
