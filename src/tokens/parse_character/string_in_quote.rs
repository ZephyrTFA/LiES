use crate::{
    dm_preprocessor::tokenize_state::TokenizeState, tokens::token_action::TokenAction,
    util::count_backslashes,
};

pub fn handle_string_in_quote(
    state: &mut TokenizeState,
    char: char,
    quote_char: char,
    current_token: &str,
) -> TokenAction {
    if char == quote_char && count_backslashes(current_token) % 2 == 0 {
        if state.multiline_string() {
            return TokenAction::ContinueToken;
        }

        state.set_in_quote(None);
        TokenAction::StartNewToken
    } else {
        match char {
            '[' if state.in_quote() == Some(&'"')
                && !state.string_literal()
                && count_backslashes(current_token) % 2 == 0 =>
            {
                state.increment_string_interop_count();
                TokenAction::IsolateToken
            }
            '}' if state.multiline_string() && current_token.ends_with(quote_char) => {
                state.set_in_quote(None);
                state.set_multiline_string(false);
                TokenAction::IsolateToken
            }
            _ => TokenAction::ContinueToken,
        }
    }
}
