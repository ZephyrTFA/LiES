use std::{collections::VecDeque, mem::take};

use log::warn;

use super::token::Token;

fn consume_string_interop(
    line: usize,
    column: usize,
    chars: &mut VecDeque<char>,
) -> VecDeque<Token> {
    if chars.front().is_some_and(|c| *c == '"') {
        return consume_string(line, column, chars.pop_front().unwrap(), chars);
    }

    VecDeque::default()
}

pub(super) fn consume_string(
    line: usize,
    mut column: usize,
    quote_start_char: char,
    chars: &mut VecDeque<char>,
) -> VecDeque<Token> {
    let mut current_str = VecDeque::default();
    current_str.push_back(Token::new(quote_start_char.into(), true, line, column));

    let mut current = vec![];
    let mut current_start = column;

    macro_rules! push_token {
        () => {
            current_str.push_back(Token::new(
                String::from_iter(take(&mut current).iter()),
                true,
                line,
                current_start,
            ));
            current_start = column;
        };
    }

    while let Some(next) = chars.pop_front() {
        column += 1;

        match next {
            _ if next == quote_start_char => {
                push_token!();
                current_str.push_back(Token::new(quote_start_char.into(), true, line, column));
                break;
            }
            '[' => {}
            _ => {
                current.push(next);
            }
        }
        current.push(next);
    }

    current_str
}
