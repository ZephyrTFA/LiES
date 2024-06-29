use std::{collections::VecDeque, fmt::Display};

use log::{error, warn};

use crate::{
    preprocess::PreprocessState,
    tokenize::token::Token,
    util::parse_error::{ParseError, ParseErrorCode},
};

#[derive(PartialEq, Eq)]
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
        let value = value.value().as_str();
        Ok(match value {
            "include" => Self::Include,
            "warn" => Self::Warn,
            "error" => Self::Error,
            "define" => Self::Define,
            "if" => Self::If,
            "ifdef" => Self::IfDef,
            "ifndef" => Self::IfNDef,
            "else" => Self::Else,
            "endif" => Self::EndIf,
            _ => {
                error!("unknown directive `{value}`");
                return Err(ParseError::new(ParseErrorCode::UnknownDirective));
            }
        })
    }
}

impl PreprocessState {
    pub fn do_directive(&mut self, mut tokens: VecDeque<Token>) -> Result<(), ParseError> {
        let init_token;
        if let Some(initial) = tokens.pop_front() {
            if initial.value() != "#" {
                error!("Attempting to parse directive without initial #?");
                return Err(ParseError::new(ParseErrorCode::UnexpectedToken)
                    .with_preprocessor_state(self, &initial));
            }
            init_token = (initial.line(), initial.column());
        } else {
            error!("Attempting to parse directive with no remaining tokens!");
            return Err(ParseError::new(ParseErrorCode::Internal));
        }

        // grab the directive
        let directive = tokens.pop_front();
        if directive.is_none() {
            let current_file = self
                .environment()
                .current_file()
                .expect("somehow no active file");
            return Err(
                ParseError::new(ParseErrorCode::UnexpectedEOL).with_file_data(
                    current_file.path(),
                    current_file.full_path(),
                    init_token.0,
                    init_token.1,
                ),
            );
        }
        let directive_token = directive.unwrap();
        let directive = Directive::try_from(&directive_token)?;

        // special logic to check for defined(XXX)
        for index in tokens
            .iter_mut()
            .enumerate()
            .filter(|tok| tok.1.value() == "defined")
            .map(|(index, _)| index)
            .rev()
            .collect::<Vec<_>>()
        {
            if !tokens.get(index + 1).is_some_and(|tok| tok.value() == "(")
                || !tokens.get(index + 3).is_some_and(|tok| tok.value() == ")")
            {
                continue;
            }
            tokens.remove(index); // defined
            tokens.remove(index); // (
            let token = tokens.remove(index).unwrap();
            if self
                .environment()
                .defines()
                .get_define(token.value())
                .is_some()
            {
                tokens[index] = "1".into();
            } else {
                tokens[index] = "0".into();
            }
        }

        // special logic for directive while active skip level
        if self.directive_skip_level > 0 {
            if self.directive_skip_level == 1 && directive == Directive::Else {
                self.decrement_directive_skip_level();
                return Ok(());
            }
            match directive {
                Directive::EndIf => {
                    self.decrement_directive_skip_level();
                }
                Directive::If | Directive::IfDef | Directive::IfNDef => {
                    self.increment_directive_skip_level();
                }
                Directive::Define
                | Directive::Warn
                | Directive::Error
                | Directive::Include
                | Directive::Else => {}
            }
            return Ok(());
        }

        match directive {
            Directive::Include => self.handle_directive_include(tokens),
            Directive::Warn => Ok(warn!(
                "Compiler Warning: {}",
                tokens
                    .into_iter()
                    .map(|tok| tok.value().clone())
                    .collect::<Vec<String>>()
                    .join("")
            )),
            Directive::Error => {
                error!(
                    "Compiler Warning: {}",
                    tokens
                        .into_iter()
                        .map(|tok| tok.value().clone())
                        .collect::<Vec<String>>()
                        .join("")
                );
                Err(ParseError::new(ParseErrorCode::ForcedError))
            }
            Directive::Define => self.handle_directive_define(tokens),
            Directive::If => self.handle_directive_if(tokens),
            Directive::IfDef => self.handle_directive_ifdef(tokens),
            Directive::IfNDef => self.handle_directive_ifndef(tokens),
            Directive::Else => {
                self.increment_directive_skip_level();
                Ok(())
            }
            Directive::EndIf => Ok(()),
        }
        .map_err(|err| err.with_preprocessor_state_if_not_set(self, &directive_token))?;

        Ok(())
    }
}
