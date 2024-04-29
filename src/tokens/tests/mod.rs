use crate::dm_preprocessor::lib::DmPreProcessor;

use super::dm_token::DmToken;

mod comment;
mod comment_multiline;
mod comment_multiline_bad;
mod condense;
mod default_token_action;
mod empty;
mod hard_lines;
mod interop_nested;
mod multi_empty;
mod multi_line;
mod quote_interior;
mod single_line;
mod string_interop;
mod unmatched_quotes;

impl DmPreProcessor {
    pub fn test_tokenize(&mut self, lines: &[&str]) -> Vec<DmToken> {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        self.tokenize_state.set_lines(&lines);
        self.start_tokenize()
    }
}
