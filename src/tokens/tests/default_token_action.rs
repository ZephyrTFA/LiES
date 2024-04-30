#[cfg(test)]
mod tests {
    use crate::tokens::{
        constants::{SYM_MATH, SYM_OTHR, SYM_QUTE},
        token_action::TokenAction,
        tokenize::defaults::handle_defaults,
    };

    #[test]
    fn test_special_symbols() {
        let mut symbols = vec![];
        symbols.extend(SYM_MATH);
        symbols.extend(SYM_OTHR);

        for symbol in symbols {
            assert!(handle_defaults(symbol, "text") == TokenAction::StartNewToken);
            assert!(
                handle_defaults(symbol, symbol.to_string().as_str()) == TokenAction::ContinueToken
            );
        }
    }

    #[test]
    fn test_quote_symbols() {
        for symbol in SYM_QUTE {
            assert!(handle_defaults(*symbol, "text") == TokenAction::IsolateToken);
            assert!(
                handle_defaults(*symbol, symbol.to_string().as_str()) == TokenAction::IsolateToken
            );
        }
    }

    #[test]
    fn test_whitespace_transition() {
        assert!(handle_defaults(' ', "text") == TokenAction::StartNewToken);
        assert!(handle_defaults(' ', " ") == TokenAction::ContinueToken);
        assert!(handle_defaults('\t', "") == TokenAction::ContinueToken);
        assert!(handle_defaults(' ', "\t") == TokenAction::StartNewToken);
    }

    #[test]
    fn test_digit_transition() {
        assert!(handle_defaults('1', "text") == TokenAction::ContinueToken);
        assert!(handle_defaults('2', "123") == TokenAction::ContinueToken);
        assert!(handle_defaults('a', "123") == TokenAction::ContinueToken);
    }

    #[test]
    fn test_no_transition() {
        assert!(handle_defaults('a', "text") == TokenAction::ContinueToken);
        assert!(handle_defaults('3', "12") == TokenAction::ContinueToken);
        assert!(handle_defaults('-', "-") == TokenAction::ContinueToken);
    }

    #[test]
    fn test_underscores_in_alphanumeric() {
        assert!(handle_defaults('_', "text") == TokenAction::ContinueToken);
        assert!(handle_defaults('_', "text1") == TokenAction::ContinueToken);
        assert!(handle_defaults('_', "text2_") == TokenAction::ContinueToken);
        assert!(handle_defaults('_', "$") == TokenAction::StartNewToken);
    }
}
