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
    state.set_in_string_special_escape(true);
    TokenAction::EndToken
}

pub fn handle_string_special_escape(state: &mut TokenizeState, char: char) -> TokenAction {
    if char
        != *state
            .in_quote()
            .expect("we are in a string special escape without a quote character")
    {
        return TokenAction::ContinueToken;
    }

    if state.multiline_string() && !state.next_char_peek().is_some_and(|c| *c == '}') {
        return TokenAction::ContinueToken;
    }

    state.set_in_quote(None);
    state.set_in_string_special_escape(false);
    if state.multiline_string() {
        assert_eq!(
            state
                .next_char()
                .expect("unexpected end of line in multiline string special escape"),
            '}'
        );
        state.set_multiline_string(false);
    }
    TokenAction::IsolateToken
}
