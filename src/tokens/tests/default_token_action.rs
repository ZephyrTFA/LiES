#[cfg(test)]
mod tests {
    use crate::tokens::{
        constants::{SYM_MATH, SYM_OTHR, SYM_QUTE},
        default_token_action::get_default_token_action,
        token_action::TokenAction,
    };

    #[test]
    fn test_special_symbols() {
        let mut symbols = vec![];
        symbols.extend(SYM_MATH);
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
    fn test_quote_symbols() {
        for symbol in SYM_QUTE {
            assert!(get_default_token_action(*symbol, "text") == TokenAction::IsolateToken);
            assert!(
                get_default_token_action(*symbol, symbol.to_string().as_str())
                    == TokenAction::IsolateToken
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

    #[test]
    fn test_underscores_in_alphanumeric() {
        assert!(get_default_token_action('_', "text") == TokenAction::ContinueToken);
        assert!(get_default_token_action('_', "text1") == TokenAction::ContinueToken);
        assert!(get_default_token_action('_', "text2_") == TokenAction::ContinueToken);
        assert!(get_default_token_action('_', "$") == TokenAction::StartNewToken);
    }
}
