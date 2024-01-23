use ::log::error;

use crate::util::log;

use super::determine_token_action::TokenAction;

const SYM_MATH: &[char; 8] = &['+', '-', '*', '/', '%', '^', '&', '|'];
const SYM_QUTE: &[char; 2] = &['"', '\''];
const SYM_OTHR: &[char; 6] = &['!', '=', '.', '#', '<', '>'];

// Determines if a new token should be started based on the current character and the current token.
pub fn get_default_token_action(char: char, current_token: &str) -> TokenAction {
    if current_token.is_empty() {
        return TokenAction::ContinueToken;
    }

    if current_token.ends_with('\\') {
        return TokenAction::EndToken;
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

    if SYM_MATH.contains(&char) || SYM_QUTE.contains(&char) || SYM_OTHR.contains(&char) {
        return if current_token.ends_with(char) {
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

    if current_token.ends_with(|char: char| char.is_alphanumeric()) {
        return if char.is_alphanumeric() {
            TokenAction::ContinueToken
        } else {
            TokenAction::StartNewToken
        };
    }

    TokenAction::StartNewToken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_symbols() {
        let mut symbols = vec![];
        symbols.extend(SYM_MATH);
        symbols.extend(SYM_QUTE);
        symbols.extend(SYM_OTHR);

        for symbol in symbols {
            assert!(get_default_token_action(symbol, "text") == TokenAction::StartNewToken);
            assert!(
                get_default_token_action(symbol, symbol.to_string().as_str())
                    == TokenAction::ContinueToken
            );
        }
    }

    #[test]
    fn test_whitespace_transition() {
        assert!(get_default_token_action(' ', "text") == TokenAction::StartNewToken);
        assert!(get_default_token_action(' ', " ") == TokenAction::ContinueToken);
        assert!(get_default_token_action('\t', "") == TokenAction::ContinueToken);
        assert!(get_default_token_action(' ', "\t") == TokenAction::StartNewToken);
    }

    #[test]
    fn test_digit_transition() {
        assert!(get_default_token_action('1', "text") == TokenAction::ContinueToken);
        assert!(get_default_token_action('2', "123") == TokenAction::ContinueToken);
        assert!(get_default_token_action('a', "123") == TokenAction::ContinueToken);
    }

    #[test]
    fn test_no_transition() {
        assert!(get_default_token_action('a', "text") == TokenAction::ContinueToken);
        assert!(get_default_token_action('3', "12") == TokenAction::ContinueToken);
        assert!(get_default_token_action('-', "-") == TokenAction::ContinueToken);
    }
}
