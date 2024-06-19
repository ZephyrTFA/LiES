use std::iter::Peekable;

use crate::{
    preprocess::PreprocessState,
    tokenize::token::Token,
    util::parse_error::{ParseError, ParseErrorCode},
};

impl PreprocessState {
    pub fn do_directive(
        &mut self,
        mut tokens: Peekable<impl Iterator<Item = Token>>,
    ) -> Result<(), ParseError> {
        // assert, this is not a parsing error this is an implementation error
        let pound_symbol = tokens.next().expect("do_directive without any tokens");
        assert!(pound_symbol.value() == "#");

        // grab the directive
        let directive = tokens.next();
        if directive.is_none() {
            return Err(ParseError::new(ParseErrorCode::UnexpectedEOL)
                .with_token(self.environment().current_file().unwrap(), &pound_symbol));
        }

        match directive.unwrap().value().as_str() {
            "include" => self.handle_directive_include(tokens),
            "define" => Ok(()),
            _ => Ok(()),
        }?;

        Ok(())
    }
}
