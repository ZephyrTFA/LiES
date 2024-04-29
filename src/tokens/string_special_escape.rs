use crate::dm_preprocessor::tokenize_state::TokenizeState;

use super::token_action::TokenAction;

pub fn get_string_special_escape_action(
    char: char,
    _current_token: &str,
    state: &mut TokenizeState,
) -> TokenAction {
    assert_eq!(char, '@');

    let mut next_char = state.next_char().expect("Unexpected end of line after '@'");
    if next_char == '{' {
        state.set_multiline_string(true);
        next_char = state
            .next_char()
            .expect("Unexpected end of line after '@{'");
    }

    state.set_in_quote(Some(next_char));
    TokenAction::EndToken
}
