use std::collections::VecDeque;

use crate::tokens::dm_token::DmToken;

pub mod dm_types;
pub mod lib;
pub mod parsing;

pub fn consume_spacing(tokens: &mut VecDeque<DmToken>) {
    while let Some(token) = tokens.front() {
        if token.value().chars().all(|c| [' ', '\t'].contains(&c)) {
            tokens.pop_front();
        } else {
            break;
        }
    }
}
