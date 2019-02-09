use super::Message;
use crate::terminal::{StyleDescriptor};
use super::TextBuffer;
use super::HighlightEngine;

use std::thread;
use std::time::Duration;
use std::sync::{mpsc};
use syntect::parsing::{ParseState, ScopeStack};
use syntect::highlighting::{Highlighter, HighlightState, HighlightIterator, Style};

static CACHE_RANGE: usize = 100;

#[derive(Clone)]
pub struct HighlightCache {
    highlight_state: HighlightState,
    parse_state: ParseState,
}

pub trait SyntaxHighlight {
    fn start_highlight_thread(&mut self, highlightengine: &HighlightEngine);
    fn highlight_from(&self, index: u32);
}

impl SyntaxHighlight for TextBuffer {
    fn start_highlight_thread(&mut self, highlightengine: &HighlightEngine) {
        let ps = highlightengine.ps.clone();
        let file_path = { self.file_path.clone() };
        let theme = highlightengine.theme.clone();
        let lines = self.lines.clone();
        let messagesender = self.messagesender.clone();
        let buffer_index = self.buffer_index.clone();
        let (highlight_send, highlight_recv) = mpsc::channel();
        self.highlight_send = Some(highlight_send);
        let syntax = match ps.find_syntax_for_file(&file_path) {
            Ok(o) => {
                match o {
                    Some(s) => s,
                    None => ps.find_syntax_plain_text()
                }
            },
            Err(_) => ps.find_syntax_plain_text()
        }.clone();
        self.syntax_name = syntax.name.clone();

        thread::spawn(move || {
            let highlighter = Highlighter::new(&theme);
            let initial_highlight_state = HighlightState::new(&highlighter, ScopeStack::new());
            let initial_parse_state = ParseState::new(&syntax);

            let mut current_line_num: usize = 0;
            let mut current_state = HighlightCache {
                highlight_state: initial_highlight_state,
                parse_state: initial_parse_state,
            };
            let mut state_cache = vec![current_state.clone()];

            let mut target_line_num: usize = 0;

            loop {
                /*
                 * Check Refresh Signal
                 */
                match highlight_recv.try_recv() {
                    Ok((request_line_num, target_line_num_local)) => {
                        if (request_line_num as usize) < current_line_num {
                            /*
                            * Update cureent_line_num and current_state here.
                            */
                            let mut request_cache_point = (request_line_num / CACHE_RANGE as u32) as usize;
                            if request_cache_point >= state_cache.len() {
                                request_cache_point = state_cache.len() - 1;
                            }
                            current_line_num = request_cache_point * CACHE_RANGE;
                            current_state = state_cache[request_cache_point].clone();
                        }
                        target_line_num = target_line_num_local as usize;
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        /*
                         * If the paired sender variable is dropped,
                         * this thread is no longer useful.
                         *
                         * This is how a texteditor kills
                         * the syntax highlighting thread for a buffer.
                         * (Start a new highlighting thread for the same buffer)
                         */
                        break;
                    }
                    _ => {},
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
                let cache_point = (current_line_num / CACHE_RANGE) as usize;
                let cache_offset = current_line_num - (cache_point * CACHE_RANGE);
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

                let mut style_descriptors = Vec::new();
                for (style, substring) in ranges.iter() {
                    style_descriptors.push(StyleDescriptor::from(*style, substring.len()));
                }

                { lines.lock().unwrap()[current_line_num].styles_cache = style_descriptors; }

                /*
                 * Check if needed to consult the caller
                 */
                if current_line_num == target_line_num || current_line_num == lines_len-1 {
                    messagesender.send(Message::HighlightReady(buffer_index)).unwrap();
                }

                current_line_num += 1;
            }
        });
    }

    fn highlight_from(&self, index: u32) {
        if let Some(highlight_send) = self.highlight_send.clone() {
            highlight_send.send((index, self.top_line+self.view_height)).unwrap();
        }
    }
}
