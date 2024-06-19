use std::{fmt::Display, iter::Peekable};

use crate::{
    preprocess::PreprocessState,
    tokenize::token::Token,
    util::parse_error::{ParseError, ParseErrorCode},
};

enum Directive {
    Include,
    Warn,
    Error,
    Define,
    If,
    IfDef,
    IfNDef,
    Else,
    EndIf,
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Include => write!(f, "#include"),
            Self::Warn => write!(f, "#warn"),
            Self::Error => write!(f, "#error"),
            Self::Define => write!(f, "#define"),
            Self::If => write!(f, "#if"),
            Self::IfDef => write!(f, "#ifdef"),
            Self::IfNDef => write!(f, "#ifndef"),
            Self::Else => write!(f, "#else"),
            Self::EndIf => write!(f, "#endif"),
        }
    }
}

impl<'a> TryFrom<&'a Token> for Directive {
    type Error = ParseError;
    fn try_from(value: &'a Token) -> Result<Self, ParseError> {
        Ok(match value.value().as_str() {
            "include" => Self::Include,
            "warn" => Self::Warn,
            "error" => Self::Error,
            "define" => Self::Define,
            "if" => Self::If,
            "ifdef" => Self::IfDef,
            "ifndef" => Self::IfNDef,
            "else" => Self::Else,
            "endif" => Self::EndIf,
            _ => return Err(ParseError::new(ParseErrorCode::UnknownDirective)),
        })
    }
}

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
                .with_preprocessor_state(self, &pound_symbol));
        }
        let directive_token = directive.unwrap();
        let directive = Directive::try_from(&directive_token)?;

        // special logic for directive while active skip level
        if self.directive_skip_level > 0 {
            todo!("directive with skip level");
        }

        match directive {
            Directive::Include => self.handle_directive_include(tokens),
            Directive::Define => self.handle_directive_define(tokens),
            Directive::Else => {
                self.increment_directive_skip_level();
                Ok(())
            }
            _ => todo!("{}", directive),
        }
        .map_err(|err| err.with_preprocessor_state_if_not_set(self, &directive_token))?;

        Ok(())
    }
}
