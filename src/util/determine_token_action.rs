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
    if current_token.is_empty() {
        return TokenAction::ContinueToken;
    }

    match in_quote {
        Some(quote_char) => {
            if char == quote_char && !current_token.ends_with('\\') {
                TokenAction::ChangeQuoteState(None)
            } else {
                TokenAction::ContinueToken
            }
        }
        None => match char {
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
            '#' => {
                if !current_token.ends_with(char) {
                    TokenAction::StartNewToken
                } else {
                    TokenAction::ContinueToken
                }
            }
            _ => {
                if should_start_new_token(char, current_token) {
                    TokenAction::StartNewToken
                } else {
                    TokenAction::ContinueToken
                }
            }
        },
    }
}
