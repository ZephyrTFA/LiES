use crate::{dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction};

pub fn handle_open_bracket(state: &mut TokenizeState) -> TokenAction {
    if state.in_comment_any() || state.string_literal() {
        return TokenAction::None;
    }

    state.increment_unmatched_brackets();
    TokenAction::IsolateToken
}
