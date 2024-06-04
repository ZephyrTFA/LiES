use std::{fmt::Display, iter::Peekable};

use log::error;

use crate::{
    tokens::dm_token::DmToken,
    util::{is_valid_identifier, ParseError},
};

#[derive(Clone)]
pub struct DmTypePath {
    parts: Vec<String>,
}

impl From<&String> for DmTypePath {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for DmTypePath {
    fn from(value: &str) -> Self {
        Self {
            parts: value.split('/').map(|str| str.to_string()).collect(),
        }
    }
}

impl Display for DmTypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.parts.join("/"))
    }
}

impl DmTypePath {
    pub fn parts(&self) -> &Vec<String> {
        &self.parts
    }

    pub fn parent(&self) -> Self {
        Self {
            parts: self.parts.iter().cloned().rev().skip(1).rev().collect(),
        }
    }

    pub fn join(&self, other: &Self) -> Self {
        let mut parts = self.parts.clone();
        parts.extend(other.parts.iter().cloned());
        Self { parts }
    }
}

impl DmTypePath {
    pub fn consume_from_tokens(
        tokens: &mut Peekable<impl Iterator<Item = DmToken>>,
    ) -> Result<DmTypePath, ParseError> {
        let mut parts = vec![];

        // important to note that typepaths CAN start with `/` but are not required to
        if tokens.peek().is_some_and(|t| t.value() == "/") {
            tokens.next();
        }

        loop {
            let token = tokens.peek();
            if token.is_none() {
                return Err(ParseError::UNEXPECTED_EOL);
            }
            let token = tokens.next().unwrap().value().to_string();
            if !is_valid_identifier(&token) {
                error!(
                    "failed to consume type path. `{}` is not a valid ident.",
                    token.escape_debug()
                );
                error!("next 7 tokens: {:#?}", tokens.take(7).collect::<Vec<_>>());
                return Err(ParseError::INVALID_IDENTIFIER);
            }
            parts.push(token);

            if !tokens
                .peek()
                .is_some_and(|tok| matches!(tok.value(), "/" | "."))
            {
                break;
            }
            tokens.next(); // consume the path seperator
        }

        assert!(!parts.is_empty(), "empty parts after loop");
        Ok(DmTypePath { parts })
    }
}
