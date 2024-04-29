use crate::{dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction};

pub fn handle_star(state: &mut TokenizeState, current_token: &str) -> TokenAction {
    if state.in_comment_single() {
        return TokenAction::None;
    }

    if state.in_comment_multi() {
        return match current_token.chars().next_back() {
            Some('/') => {
                state.increment_comment_multi();
                TokenAction::EndToken
            }
            _ => TokenAction::None,
        };
    }

    if state.is_last_token_an_escape() {
        return TokenAction::None;
    }

    match current_token.chars().next_back() {
        Some('/') => {
            state.increment_comment_multi();
            TokenAction::EndToken
        }
        _ => TokenAction::None,
    }
}
