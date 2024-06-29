use std::collections::VecDeque;

use crate::{
    preprocess::{define::definition::DefineDefinition, PreprocessState},
    tokenize::token::Token,
    util::parse_error::{ParseError, ParseErrorCode},
};

use super::DirectiveResult;

impl PreprocessState {
    pub(super) fn handle_define_undef(&mut self, mut tokens: VecDeque<Token>) -> DirectiveResult {
        // consume whitespace
        while tokens.front().is_some_and(|next| next.is_only_spacing()) {
            tokens.pop_front();
        }

        let name_token = tokens.pop_front();
        if name_token.is_none() {
            return Err(ParseError::new(ParseErrorCode::UnexpectedEOL));
        }
        let name_token = name_token.unwrap();
        let define_name = name_token.value();

        self.environment_mut()
            .defines_mut()
            .remove_define(define_name);
        Ok(())
    }

    pub(super) fn handle_directive_define(
        &mut self,
        mut tokens: VecDeque<Token>,
    ) -> DirectiveResult {
        // consume whitespace
        while tokens.front().is_some_and(|next| next.is_only_spacing()) {
            tokens.pop_front();
        }

        let define_name = tokens.pop_front().map(|tok| tok.value().clone());
        if define_name.is_none() {
            return Err(ParseError::new(ParseErrorCode::UnexpectedEOL));
        }
        let define_name = define_name.unwrap();

        if tokens.front().is_some_and(|token| token.value() == "(") {
            return self.handle_define_macro(tokens);
        }

        while tokens.front().is_some_and(|next| next.is_only_spacing()) {
            tokens.pop_front();
        }
        self.environment_mut()
            .defines_mut()
            .insert_define(DefineDefinition::new_define(
                &define_name,
                tokens.into_iter().collect(),
            ));
        Ok(())
    }
}
