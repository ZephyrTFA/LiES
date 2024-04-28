use super::{
    constants::{SYM_MATH, SYM_OTHR, SYM_QUTE},
    token_action::TokenAction,
};

// Determines if a new token should be started based on the current character and the current token.
pub fn get_default_token_action(char: char, current_token: &str) -> TokenAction {
    // special case for quotes, they should always be isolated
    if SYM_QUTE.contains(&char) {
        return TokenAction::IsolateToken;
    }

    if current_token.is_empty() {
        return TokenAction::ContinueToken;
    }

    if char.is_whitespace() {
        return if current_token.ends_with(char) {
            TokenAction::ContinueToken
        } else {
            TokenAction::StartNewToken
        };
    }

    const COMMENT_SYMBOLS: &[char; 2] = &['/', '*'];
    if COMMENT_SYMBOLS.contains(&char) {
        return if current_token.ends_with(COMMENT_SYMBOLS) {
            TokenAction::ContinueToken
        } else {
            TokenAction::StartNewToken
        };
    }

    if SYM_MATH.contains(&char) {
        return if current_token.ends_with(SYM_MATH) {
            TokenAction::ContinueToken
        } else {
            TokenAction::StartNewToken
        };
    }

    if SYM_OTHR.contains(&char) {
        return if current_token.ends_with(SYM_OTHR) {
            TokenAction::ContinueToken
        } else {
            TokenAction::StartNewToken
        };
    }

    if current_token.ends_with(char::is_whitespace) {
        return if current_token.ends_with(char) {
            TokenAction::ContinueToken
        } else {
            TokenAction::StartNewToken
        };
    }

    static IS_VALID_IDENT_CHAR: fn(char) -> bool =
        |char: char| char.is_alphanumeric() || char == '_';
    if current_token.ends_with(IS_VALID_IDENT_CHAR) {
        return if IS_VALID_IDENT_CHAR(char) {
            TokenAction::ContinueToken
        } else {
            TokenAction::StartNewToken
        };
    }

    TokenAction::StartNewToken
}
