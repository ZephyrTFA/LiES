use crate::dm_preprocessor::tokenize_state::TokenizeState;

mod at;
mod close_bracket;
pub mod defaults;
mod hash;
mod open_bracket;
mod quotes;
mod slash;
mod special_escape;
mod star;
mod string_in_quote;

use at::handle_at;
use close_bracket::handle_close_bracket;
use defaults::handle_defaults;
use hash::handle_hash;
use open_bracket::handle_open_bracket;
use quotes::handle_quotes;
use slash::handle_slash;
use special_escape::handle_string_special_escape;
use star::handle_star;
use string_in_quote::handle_string_in_quote;

use super::token_action::TokenAction;

pub fn parse_character(state: &mut TokenizeState, char: char, current_token: &str) -> TokenAction {
    if state.in_string_special_escape() {
        return handle_string_special_escape(state, char);
    }

    if let Some(quote_char) = state.in_quote() {
        return handle_string_in_quote(state, char, *quote_char, current_token);
    }

    let mut token_result = match char {
        ']' => handle_close_bracket(state, current_token),
        '[' => handle_open_bracket(state),
        '"' | '\'' => handle_quotes(state, char, current_token),
        '#' => handle_hash(state),
        '*' => handle_star(state, current_token),
        '/' => handle_slash(state, current_token),
        '@' => handle_at(state, current_token),
        _ => handle_defaults(char, current_token),
    };

    if token_result == TokenAction::None {
        token_result = handle_defaults(char, current_token);
    }

    token_result
}
