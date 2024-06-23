use std::iter::Peekable;

use crate::{preprocess::PreprocessState, tokenize::token::Token};

use super::DirectiveResult;

impl PreprocessState {
    pub(super) fn handle_directive_ifndef(
        &mut self,
        mut tokens: Peekable<impl Iterator<Item = Token>>,
    ) -> DirectiveResult {
        while tokens.peek().is_some_and(|tok| tok.is_only_spacing()) {
            tokens.next();
        }

        let define_name = tokens.next().unwrap();
        if self
            .environment()
            .defines()
            .get_define(define_name.value())
            .is_some()
        {
            self.increment_directive_skip_level();
        }

        Ok(())
    }
}
