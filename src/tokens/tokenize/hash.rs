use crate::{
    dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction,
    util::whitespace_char::is_first_non_whitespace_char,
};

pub fn handle_hash(state: &mut TokenizeState) -> TokenAction {
    if is_first_non_whitespace_char(state.line_tokens()) {
        state.set_in_preprocessor(true);
        return TokenAction::IsolateToken;
    }

    TokenAction::None
}
