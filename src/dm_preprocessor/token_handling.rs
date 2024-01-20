use std::fmt::Display;

use log::{debug, error};
use once_cell::sync::Lazy;
use regex::Regex;

use super::DmPreProcessor;

#[derive(Debug, Clone)]
pub struct DmToken {
    value: String,
}

impl Display for DmToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl DmToken {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PartialEq for DmToken {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl DmPreProcessor {
    fn condense_lines(&mut self, lines: Vec<impl Into<String>>) -> Vec<String> {
        let mut lines = lines
            .into_iter()
            .map(|line| line.into())
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return lines;
        }

        let mut condensed = vec![];

        while !lines.is_empty() {
            let mut line = lines.remove(0);

            while line.ends_with('\\') {
                line.pop();
                if !lines.is_empty() {
                    line.push_str(&lines.remove(0));
                }
            }

            condensed.push(line);
        }

        condensed
    }

    pub fn tokenize(&mut self, lines: Vec<impl Into<String>>) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = self.condense_lines(lines);
        let mut tokens: Vec<DmToken> = vec![];

        let mut in_quote: Option<char> = None;
        for line in condensed_lines {
            let mut token = String::new();
            for char in line.chars() {
                if token.is_empty() {
                    token.push(char);
                    continue;
                }

                if let Some(quote_char) = in_quote {
                    if char != quote_char || token.ends_with('\\') {
                        token.push(char);
                    } else {
                        tokens.push(DmToken::new(token));
                        tokens.push(DmToken::new(quote_char.to_string()));
                        token = String::new();
                        in_quote = None;
                    }
                    continue;
                }

                if !token.ends_with('\\') {
                    match char {
                        '"' | '\'' => {
                            if !token.is_empty() {
                                tokens.push(DmToken::new(token));
                                token = String::new();
                            }
                            tokens.push(DmToken::new(char.to_string()));
                            in_quote = Some(char);
                            continue;
                        }
                        _ => {}
                    }
                }

                match char {
                    ' ' | '\t' => {
                        if token.ends_with(char) {
                            token.push(char);
                            continue;
                        } else if !token.is_empty() {
                            tokens.push(DmToken::new(token));
                            token = String::new();
                            token.push(char);
                            continue;
                        }
                    }
                    _ => {
                        if token.ends_with(&[' ', '\t']) {
                            tokens.push(DmToken::new(token));
                            token = String::new();
                            token.push(char);
                            continue;
                        }
                    }
                }

                static IS_IDENT_CHAR: fn(char) -> bool =
                    |char| char.is_ascii_alphanumeric() || char == '_';
                if IS_IDENT_CHAR(char) && token.ends_with(IS_IDENT_CHAR) {
                    token.push(char);
                    continue;
                }

                static IS_NUMBER_CHAR: fn(char) -> bool = |char| char.is_ascii_digit();
                if IS_NUMBER_CHAR(char) && token.ends_with(IS_NUMBER_CHAR) {
                    token.push(char);
                    continue;
                }

                static IS_OPERATOR_CHAR: fn(char) -> bool = |char| {
                    matches!(
                        char,
                        '+' | '-' | '*' | '/' | '%' | '^' | '&' | '|' | '=' | '<' | '>' | '!'
                    )
                };
                if IS_OPERATOR_CHAR(char) && token.ends_with(IS_OPERATOR_CHAR) {
                    token.push(char);
                    continue;
                }

                tokens.push(DmToken::new(token));
                token = String::new();
                token.push(char);
            }

            if !token.is_empty() {
                tokens.push(DmToken::new(token));
            }
            tokens.push(DmToken::new("\n".into()));
        }

        tokens
    }
}
