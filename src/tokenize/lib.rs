use std::{collections::VecDeque, fs::read_to_string, path::Path};

use log::error;

use crate::{
    tokenize::string::consume_string,
    util::parse_error::{ParseError, ParseErrorCode},
};

use super::token::{FileTokens, Token};

const MATH_OPERATORS: [char; 9] = ['+', '-', '/', '*', '=', '^', '|', '>', '<'];

fn is_ident_char(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

fn can_be_ident(tokens: &VecDeque<char>) -> bool {
    tokens
        .front()
        .is_some_and(|c| c.is_alphabetic() || *c == '_')
        && tokens
            .iter()
            .skip(1)
            .all(|c| c.is_alphanumeric() || *c == '_')
}

pub fn tokenize_line(line: &str, line_idx: usize) -> Result<Vec<Token>, ParseError> {
    let mut line_tokens = vec![];
    let mut token_chars: VecDeque<char> = VecDeque::new();
    let mut char_idx = 0;

    macro_rules! push_token {
        () => {
            if !token_chars.is_empty() {
                line_tokens.push(Token::new(
                    token_chars.iter().collect(),
                    false,
                    line_idx,
                    char_idx - token_chars.len(),
                ));
                token_chars.clear();
            }
        };
    }

    let mut chars: VecDeque<char> = line.chars().collect();
    while let Some(line_char) = chars.pop_front() {
        macro_rules! push_char {
            () => {
                token_chars.push_back(line_char);
                char_idx += 1;
                continue;
            };
        }

        if token_chars.is_empty() || token_chars.back().copied().unwrap() == line_char {
            push_char!();
        }

        if matches!(line_char, ' ' | '\t') {
            if !matches!(token_chars.back().unwrap(), ' ' | '\t') {
                push_token!();
            }
            push_char!();
        }

        if MATH_OPERATORS.contains(&line_char) {
            if !MATH_OPERATORS.contains(token_chars.back().unwrap()) {
                push_token!();
            }
            push_char!();
        }

        if matches!(line_char, '\'' | '"') {
            push_token!();
            line_tokens
                .append(&mut consume_string(line_idx, char_idx, line_char, &mut chars).into());
            continue;
        }

        if is_ident_char(line_char) && can_be_ident(&token_chars) {
            push_char!();
        }

        if line_char.is_numeric() {
            if !token_chars.iter().all(|c| c.is_numeric() || *c == '.') {
                push_token!();
            }
            push_char!();
        }

        if line_char.is_alphanumeric() || line_char == '_' {
            let front = token_chars.front().copied().unwrap();
            if !(front.is_alphabetic() || front == '_') {
                push_token!();
            }
            push_char!();
        }

        if line_char == '.' {
            let front = token_chars.front().copied().unwrap();
            if !(front.is_numeric() || front == '.') {
                push_token!();
            }
            push_char!();
        }

        push_token!();
        push_char!();
    }
    push_token!();

    Ok(line_tokens)
}

pub fn tokenize_lines(
    lines: impl Iterator<Item = impl Into<String>>,
    file_path: &Path,
) -> Result<FileTokens, ParseError> {
    let mut file_tokens: FileTokens = FileTokens::new(file_path.display().to_string());

    let mut line_idx = 0;
    let mut lines: VecDeque<String> = lines.map(|item| item.into()).collect();
    while let Some(line) = lines.pop_front() {
        line_idx += 1;
        file_tokens.add_tokens(tokenize_line(&line, line_idx)?.into_iter());
    }

    Ok(file_tokens)
}

pub fn tokenize_file(file: &Path) -> Result<FileTokens, ParseError> {
    let file_lines_store: String = read_to_string(file).map_err(|err| {
        error!("Failed to read file at `{file:?}`: {err}");
        ParseError::new(ParseErrorCode::Internal)
    })?;
    tokenize_lines(file_lines_store.split('\n'), file)
}
