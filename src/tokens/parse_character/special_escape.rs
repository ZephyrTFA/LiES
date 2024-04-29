use crate::{dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction};

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
