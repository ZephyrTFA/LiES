use crate::{dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction};

pub fn handle_quotes(state: &mut TokenizeState, char: char, current_token: &str) -> TokenAction {
    if state.in_comment_any() || state.in_preprocessor() {
        return if char == '"' {
            if state.in_preprocessor() {
                state.set_in_quote(Some(char));
                state.set_multiline_string(current_token.ends_with('{'));
            }
            TokenAction::IsolateToken
        } else {
            TokenAction::StartNewToken
        };
    }

    state.set_in_quote(Some(char));
    state.set_string_literal(current_token.ends_with('@'));
    state.set_multiline_string(current_token.ends_with('{'));
    state.set_next_token_is_in_string(true);
    TokenAction::IsolateToken
}
