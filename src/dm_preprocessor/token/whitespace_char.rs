use super::dm_token::DmToken;

// Returns true if the line is empty or only contains whitespace.
pub fn is_first_non_whitespace_char(line_tokens: &[DmToken]) -> bool {
    line_tokens.is_empty()
        || line_tokens
            .iter()
            .all(|token| token.value().chars().all(char::is_whitespace))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_line() {
        let line_tokens: Vec<DmToken> = vec![];
        assert!(is_first_non_whitespace_char(&line_tokens));
    }

    #[test]
    fn test_whitespace_only_line() {
        let line_tokens: Vec<DmToken> =
            vec![DmToken::from(" "), DmToken::from("\t"), DmToken::from(" ")];
        assert!(is_first_non_whitespace_char(&line_tokens));
    }

    #[test]
    fn test_non_whitespace_line() {
        let line_tokens: Vec<DmToken> = vec![
            DmToken::from(" "),
            DmToken::from("non-whitespace"),
            DmToken::from(" "),
        ];
        assert!(!is_first_non_whitespace_char(&line_tokens));
    }
}
