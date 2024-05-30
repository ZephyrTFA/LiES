use std::{collections::VecDeque, fmt::Display};

use crate::{
    dm_parser::lib::DmParser,
    tokens::dm_token::DmToken,
    util::{is_valid_identifier, ParseError},
};

use super::scope::Scope;

#[derive(Debug, Default, Clone)]
pub struct DmTypePath {
    type_path_parts: Vec<String>,
}

impl Display for DmTypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.type_path_parts.join("/"))
    }
}

impl From<&str> for DmTypePath {
    fn from(s: &str) -> Self {
        let type_path_parts = s.split('/').map(String::from).collect();
        DmTypePath { type_path_parts }
    }
}

impl From<&[&str]> for DmTypePath {
    fn from(parts: &[&str]) -> Self {
        let type_path_parts = parts.iter().map(|s| s.to_string()).collect();
        DmTypePath { type_path_parts }
    }
}

impl DmTypePath {
    pub fn global() -> &'static DmTypePath {
        static GLOBAL: DmTypePath = DmTypePath {
            type_path_parts: Vec::new(),
        };
        &GLOBAL
    }

    pub fn type_path(&self) -> String {
        format!("/{}", self.type_path_parts.join("/"))
    }

    pub fn is_global(&self) -> bool {
        self.type_path_parts.is_empty()
    }

    pub fn parent(&self) -> DmTypePath {
        if self.type_path_parts.is_empty() {
            panic!("attempt to get parent of global scope")
        }
        let mut parent = self.clone();
        parent.type_path_parts.pop();
        parent
    }

    pub fn extend(&self, extension: &str) -> DmTypePath {
        let mut extended = self.clone();
        extended.type_path_parts.push(extension.to_string());
        extended
    }

    pub fn extend_parts(&self, extension: &[&str]) -> DmTypePath {
        let parts = self
            .type_path_parts
            .iter()
            .map(String::as_str)
            .chain(extension.iter().copied())
            .filter(|s| *s != "/");
        DmTypePath {
            type_path_parts: parts.map(String::from).collect(),
        }
    }

    pub fn extend_path(&self, extension: &DmTypePath) -> DmTypePath {
        let parts: Vec<&str> = extension
            .type_path_parts
            .iter()
            .map(String::as_str)
            .collect();
        self.extend_parts(&parts)
    }

    pub fn parts(&self) -> Vec<&str> {
        self.type_path_parts
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    pub fn first(&self) -> &str {
        self.type_path_parts.first().unwrap()
    }

    pub fn last(&self) -> &str {
        self.type_path_parts.last().unwrap()
    }
}

impl DmParser {
    fn consume_path_seperators(tokens: &mut VecDeque<DmToken>) {
        while let Some(token) = tokens.front() {
            let token = token.value();
            match token {
                "/" => {
                    tokens.pop_front();
                }
                "\n" => break,
                _ if token.chars().all(char::is_whitespace) => {
                    tokens.pop_front();
                }
                _ => break,
            }
        }
    }

    pub fn consume_type_path(
        token: DmToken,
        tokens: &mut VecDeque<DmToken>,
    ) -> Result<DmTypePath, ParseError> {
        Self::consume_path_seperators(tokens);
        if !is_valid_identifier(token.value()) && token.value() != "/" {
            return Err(ParseError::INVALID_IDENTIFIER);
        }
        let mut type_path_parts = vec![token.value().to_string()];

        Self::consume_path_seperators(tokens);
        while let Some(front) = tokens.front() {
            if !is_valid_identifier(front.value()) {
                break;
            }
            type_path_parts.push(front.value().to_string());
            tokens.pop_front();
            Self::consume_path_seperators(tokens);
        }
        type_path_parts.retain(|s| s != "/");

        Ok(DmTypePath { type_path_parts })
    }

    pub(super) fn consume_scope_typepath(
        &self,
        token: DmToken,
        tokens: &mut VecDeque<DmToken>,
        scope: &mut Scope,
    ) -> Result<(), ParseError> {
        let parent_type_path = scope
            .parent()
            .as_ref()
            .and_then(|p| p.type_path())
            .unwrap_or(DmTypePath::global());

        scope.set_type_path(parent_type_path.extend_path(&Self::consume_type_path(token, tokens)?));
        println!("type_path: {}", scope.type_path().unwrap());
        Ok(())
    }
}
