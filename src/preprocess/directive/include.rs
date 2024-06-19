use std::iter::Peekable;

use crate::{
    preprocess::PreprocessState,
    tokenize::token::Token,
    util::parse_error::{ParseError, ParseErrorCode},
};

use super::DirectiveResult;

impl PreprocessState {
    pub(super) fn handle_directive_include(
        &mut self,
        mut tokens: Peekable<impl Iterator<Item = Token>>,
    ) -> DirectiveResult {
        // get rid of whitespace
        while tokens.peek().is_some_and(|tok| tok.is_only_spacing()) {
            tokens.next().unwrap();
        }

        let string_opening = tokens.next();
        if string_opening.is_none() {
            return Err(ParseError::new(ParseErrorCode::UnexpectedEOL));
        }

        let string_opening = string_opening.unwrap();
        if string_opening.value() != "\"" {
            return Err(ParseError::new(ParseErrorCode::ExpectedString)
                .with_token(self.environment().current_file().unwrap(), &string_opening));
        }

        let tokens: Vec<Token> = tokens.take_while(|tok| tok.value() != "\"").collect();
        if tokens.is_empty() {
            return Err(ParseError::new(ParseErrorCode::MalformedString)
                .with_token(self.environment().current_file().unwrap(), &string_opening));
        }

        let path = tokens
            .into_iter()
            .map(|tok| tok.value().clone())
            .collect::<Vec<String>>()
            .join("");
        self.preprocess(&path)?;

        Ok(())
    }
}
