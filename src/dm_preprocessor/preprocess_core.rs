use regex::Regex;

use crate::util::dm_file::DmFile;

use super::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub fn preprocess(&mut self, file: &DmFile) -> Vec<DmToken> {
        let tokens = self.tokenize(file.lines());
        let mut skip_until: Option<Regex> = None;

        let mut is_string = false;
        let mut is_quote = false;

        let mut final_tokens = vec![];
        for (idx, token) in tokens.into_iter().enumerate() {
            // println!("token: {:?}", token);
        }

        final_tokens
    }
}
