use log::{error, trace};

use crate::dm_preprocessor::tokenize_state::TokenizeState;
use crate::tokens::parse_character::parse_character;

use super::token_action::TokenAction;

pub fn get_token_actions(state: &mut TokenizeState, char: char, token: &str) {
    trace!("Char: `{}`", char.escape_debug());

    let next_action = parse_character(state, char, token);

    match next_action {
        TokenAction::StartNewToken => {
            if !token.is_empty() {
                state.add_line_token(token);
            }
            state.add_line_token(char.to_string());
        }
        TokenAction::ContinueToken => {
            state.add_line_token(char.to_string());
        }
        TokenAction::EndToken => {
            state.add_line_token(token);
            state.add_line_token(char.to_string());
        }
        TokenAction::IsolateToken => {
            if !token.is_empty() {
                state.add_line_token(token);
            }
            state.add_line_token(char.to_string());
        }
        _ => {
            error!(
                "Unexpected token action `{}` with char {}",
                next_action, char
            );
            panic!();
        }
    }
}
