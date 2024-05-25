use crate::{
    dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction,
    util::count_backslashes,
};

pub fn handle_close_bracket(state: &mut TokenizeState, current_token: &str) -> TokenAction {
    if state.in_comment_any() || state.string_literal() {
        return TokenAction::None;
    }

    if (current_token.is_empty() && state.is_last_token_an_escape())
        || count_backslashes(current_token) % 2 == 1
    {
        return TokenAction::None;
    } else if state.unmatched_brackets() {
        state.decrement_unmatched_brackets();
    } else if state.in_string_interop() {
        state.decrement_string_interop_count();
        state.set_next_token_is_in_string(true);
    }

    TokenAction::IsolateToken
}
