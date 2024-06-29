use std::collections::VecDeque;

use crate::{preprocess::PreprocessState, tokenize::token::Token};

use super::DirectiveResult;

impl PreprocessState {
    pub fn handle_define_macro(&mut self, mut _tokens: VecDeque<Token>) -> DirectiveResult {
        Ok(())
    }
}
