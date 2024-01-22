// Determines if a new token should be started based on the current character and the current token.
pub fn should_start_new_token(char: char, current_token: &str) -> bool {
    if current_token.ends_with('\\') || current_token.is_empty() {
        return false;
    }

    let is_special_symbol = matches!(char, '"' | '\'' | '&' | '!');
    if is_special_symbol {
        return !current_token.ends_with(char);
    }

    const MATH_OPERATORS: &[char; 10] = &['+', '-', '*', '/', '%', '^', '|', '=', '<', '>'];

    let is_math_operator = MATH_OPERATORS.contains(&char);
    if is_math_operator {
        return !current_token.ends_with(|c| MATH_OPERATORS.contains(&c));
    }

    if char.is_whitespace() {
        return !current_token.ends_with(char);
    }

    if current_token.ends_with(char::is_whitespace) {
        return true;
    }

    if current_token.ends_with('#') {
        return !current_token.ends_with(char);
    }

    let is_digit_transition =
        char.is_ascii_digit() != current_token.chars().all(|c| c.is_ascii_digit());
    if is_digit_transition {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_symbols() {
        let symbols = [
            '"', '\'', '+', '-', '*', '/', '%', '^', '&', '|', '=', '<', '>', '!',
        ];
        for &symbol in &symbols {
            assert!(
                should_start_new_token(symbol, "text"),
                "Failed at symbol: {}",
                symbol
            );
        }
    }

    #[test]
    fn test_whitespace_transition() {
        assert!(should_start_new_token(' ', "text"));
        assert!(!should_start_new_token(' ', " "));
        assert!(!should_start_new_token('\t', ""));
        assert!(should_start_new_token(' ', "\t"));
    }

    #[test]
    fn test_digit_transition() {
        assert!(should_start_new_token('1', "text"));
        assert!(!should_start_new_token('2', "123"));
        assert!(should_start_new_token('a', "123"));
    }

    #[test]
    fn test_no_transition() {
        assert!(!should_start_new_token('a', "text"));
        assert!(!should_start_new_token('3', "12"));
        assert!(!should_start_new_token('-', "-"));
    }
}
