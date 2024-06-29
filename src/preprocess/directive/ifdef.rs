use std::collections::VecDeque;

use crate::{preprocess::PreprocessState, tokenize::token::Token};

use super::DirectiveResult;

impl PreprocessState {
    pub(super) fn handle_directive_ifdef(
        &mut self,
        mut tokens: VecDeque<Token>,
    ) -> DirectiveResult {
        while tokens.front().is_some_and(|tok| tok.is_only_spacing()) {
            tokens.pop_front();
        }

        let define_name = tokens.pop_front().unwrap();
        if self
            .environment()
            .defines()
            .get_define(define_name.value())
            .is_none()
        {
            self.increment_directive_skip_level();
        }

        Ok(())
    }
}
