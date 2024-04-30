use crate::{dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction};

pub fn handle_at(state: &mut TokenizeState) -> TokenAction {
    if state.in_comment_any() {
        return TokenAction::None;
    }

    if state.is_last_token_an_escape() {
        return TokenAction::None;
    }

    get_string_special_escape_action(state)
}

fn get_string_special_escape_action(state: &mut TokenizeState) -> TokenAction {
    let mut next_char = state.next_char().expect("Unexpected end of line after '@'");
    if next_char == '{' {
        state.set_multiline_string(true);
        next_char = state
            .next_char()
            .expect("Unexpected end of line after '@{'");
    }

    state.set_in_quote(Some(next_char));
    state.set_in_string_special_escape(true);
    TokenAction::EndToken
}
