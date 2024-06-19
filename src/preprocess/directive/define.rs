use std::iter::Peekable;

use crate::{
    preprocess::{define::definition::DefineDefinition, PreprocessState},
    tokenize::token::Token,
    util::parse_error::{ParseError, ParseErrorCode},
};

use super::DirectiveResult;

impl PreprocessState {
    pub(super) fn handle_directive_define(
        &mut self,
        mut tokens: Peekable<impl Iterator<Item = Token>>,
    ) -> DirectiveResult {
        // consume whitespace
        while tokens.peek().is_some_and(|next| next.is_only_spacing()) {
            tokens.next();
        }

        let define_name = tokens.next();
        if define_name.is_none() {
            return Err(ParseError::new(ParseErrorCode::UnexpectedEOL));
        }
        let define_name = define_name.unwrap();

        if tokens.peek().is_some_and(|token| token.value() == "(") {
            todo!("macro parsing");
        }

        self.environment_mut()
            .defines_mut()
            .insert_define(DefineDefinition::new_define(
                define_name.value(),
                tokens.collect(),
            ));
        Ok(())
    }
}
