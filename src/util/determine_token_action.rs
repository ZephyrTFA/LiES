use super::start_new_token::should_start_new_token;

#[derive(Debug, PartialEq)]
pub enum TokenAction {
    StartNewToken,
    ContinueToken,
    EndToken,
    ChangeQuoteState(Option<char>),
    None,
}

// Determines what action should be taken for the current character.
pub fn determine_token_action(
    char: char,
    current_token: &str,
    in_quote: Option<char>,
) -> TokenAction {
    if let Some(quote_char) = in_quote {
        if (char == quote_char && !current_token.ends_with('\\')) {
            return TokenAction::ChangeQuoteState(None);
        } else {
            return TokenAction::ContinueToken;
        }
    }

    match char {
        '"' | '\'' => {
            if !current_token.is_empty() {
                return TokenAction::StartNewToken;
            }
            TokenAction::ChangeQuoteState(Some(char))
        }
        '.' => TokenAction::StartNewToken,
        ' ' | '\t' => {
            if current_token.ends_with(char) {
                TokenAction::ContinueToken
            } else if !current_token.is_empty() {
                TokenAction::StartNewToken
            } else {
                TokenAction::None
            }
        }
        _ => {
            if should_start_new_token(char, current_token) {
                TokenAction::StartNewToken
            } else {
                TokenAction::ContinueToken
            }
        }
    }
}
