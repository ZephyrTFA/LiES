use log::{error, trace};

use crate::dm_preprocessor::{token::determine_token_action::TokenAction, DmPreProcessor};

use super::{
    condense_braces::condense_braces, condense_lines::condense_lines,
    determine_token_action::determine_token_action, dm_token::DmToken,
};

impl DmPreProcessor {
    pub fn tokenize(&mut self, lines: &[impl Into<String> + Clone]) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = condense_lines(lines);
        let condensed_lines = condense_braces(&condensed_lines);
        let mut tokens: Vec<DmToken> = vec![];

        for line in condensed_lines {
            let mut token = String::new();
            self.tokenize_state.set_in_preprocessor(false);
            self.tokenize_state.set_comment_single(false);

            trace!("Tokenizing line: `{}`", line);
            for char in line.chars() {
                trace!("Char: `{}`", char.escape_debug());
                let next_action = determine_token_action(&mut self.tokenize_state, char, &token);
                match next_action {
                    TokenAction::StartNewToken => {
                        if !token.is_empty() {
                            self.tokenize_state.add_line_token(token);
                        }
                        token = char.to_string();
                    }
                    TokenAction::ContinueToken => {
                        token.push(char);
                    }
                    TokenAction::EndToken => {
                        token.push(char);
                        self.tokenize_state.add_line_token(token);
                        token = String::new();
                    }
                    TokenAction::IsolateToken => {
                        if !token.is_empty() {
                            self.tokenize_state.add_line_token(token);
                        }
                        self.tokenize_state.add_line_token(char.to_string());
                        token = String::new();
                    }
                    _ => {
                        error!(
                            "Unexpected token action `{}` with char {}",
                            next_action, char
                        );
                        panic!();
                    }
                }
            }

            if !token.is_empty() {
                self.tokenize_state.add_line_token(token);
            }
            self.tokenize_state.add_line_token("\n");
            tokens.append(&mut self.tokenize_state.finalize_line_tokens());
            if self.tokenize_state.in_quote().is_some() && !self.tokenize_state.in_preprocessor() {
                error!(
                    "Unterminated quote `{}` in line `{}`",
                    self.tokenize_state.in_quote().unwrap(),
                    line
                );
                panic!();
            }

            if self.tokenize_state.unmatched_brackets() {
                error!("Unmatched brackets in line `{}`", line);
                panic!();
            }

            if self.tokenize_state.in_string_interop() {
                error!("Unmatched string interop in line `{}`", line);
                panic!();
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::dm_preprocessor::{token::dm_token::DmToken, DmPreProcessor};

    #[test]
    fn test_tokenize_empty() {
        let mut preprocessor = DmPreProcessor::new();

        let lines: Vec<&str> = vec![];
        let expected = vec![];
        let result = preprocessor.tokenize(&lines);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_single_line() {
        let mut preprocessor = DmPreProcessor::new();

        let lines = vec!["This is a test."];

        let expected = vec![
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_multiple_lines() {
        let mut preprocessor = DmPreProcessor::new();

        let lines = vec!["This is a test.", "Another test."];

        let expected = vec![
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("Another"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_empty_lines() {
        let mut preprocessor = DmPreProcessor::new();

        let lines = vec!["This is a test.", ""];

        let expected: Vec<DmToken> = vec![
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_interop() {
        let mut preprocessor = DmPreProcessor::new();

        let lines = vec![
            "\"String with zero interop\"",
            "\"String with [\"one\"] interop\"",
            "\"String [\"with\"] [\"two\"] separate interops\"",
        ];

        let expected = vec![
            DmToken::from("\""),
            DmToken::from("String with zero interop"),
            DmToken::from("\""),
            DmToken::from("\n"),
            DmToken::from("\""),
            DmToken::from("String with "),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("one"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from(" interop"),
            DmToken::from("\""),
            DmToken::from("\n"),
            DmToken::from("\""),
            DmToken::from("String "),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("with"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from(" "),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("two"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from(" separate interops"),
            DmToken::from("\""),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_interop_nested() {
        let mut preprocessor = DmPreProcessor::new();
        let lines = vec!["\"String with [\"a [\"triple [\"nested\"]\"]\"] interop\""];

        let expected = vec![
            DmToken::from("\""),
            DmToken::from("String with "),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("a "),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("triple "),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("nested"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from(" interop"),
            DmToken::from("\""),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_condense_lines() {
        let mut preprocessor = DmPreProcessor::new();

        let lines = vec!["This is a test. \\", "Another test."];

        let expected = vec![
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from(" "),
            DmToken::from("Another"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_quote_interior() {
        let mut preprocesser = DmPreProcessor::new();
        let lines = vec!["\"THIS IS A QUOTE\""];

        let expected = vec![
            DmToken::from("\""),
            DmToken::from("THIS IS A QUOTE"),
            DmToken::from("\""),
            DmToken::from("\n"),
        ];

        let result = preprocesser.tokenize(&lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_comment() {
        let mut preprocessor = DmPreProcessor::new();
        let lines = vec!["This is a test.", "// This is a comment.", "Another test."];

        let expected = vec![
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("//"),
            DmToken::from(" "),
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("comment"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("Another"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_comment_multiline() {
        let mut preprocessor = DmPreProcessor::new();
        let lines = vec![
            "This is a test.",
            "/*",
            "This is a comment.",
            "*/",
            "Another test.",
        ];

        let expected = vec![
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("/*"),
            DmToken::from("\n"),
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("comment"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("*/"),
            DmToken::from("\n"),
            DmToken::from("Another"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
        ];

        let result = preprocessor.tokenize(&lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_comment_multline_commented_bad_end() {
        let mut preprocesser = DmPreProcessor::new();

        let lines = vec![
            "/*",
            " *",
            " * This is a comment.",
            " *",
            " // /*/", // this doen't end the comment because the ending is broken
            "A lone single-quote '", // unmatched quotes will fail except in comments
            "A lone double-quote \"", // these won't fail because the commend doesn't end in one of the previous line
            "*/",
        ];

        let expected = vec![
            DmToken::from("/*"),
            DmToken::from("\n"),
            DmToken::from(" "),
            DmToken::from("*"),
            DmToken::from("\n"),
            DmToken::from(" "),
            DmToken::from("*"),
            DmToken::from(" "),
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("comment"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from(" "),
            DmToken::from("*"),
            DmToken::from("\n"),
            DmToken::from(" "),
            DmToken::from("//"),
            DmToken::from(" "),
            DmToken::from("/*"),
            DmToken::from("/"),
            DmToken::from("\n"),
            DmToken::from("A"),
            DmToken::from(" "),
            DmToken::from("lone"),
            DmToken::from(" "),
            DmToken::from("single"),
            DmToken::from("-"),
            DmToken::from("quote"),
            DmToken::from(" "),
            DmToken::from("'"),
            DmToken::from("\n"),
            DmToken::from("A"),
            DmToken::from(" "),
            DmToken::from("lone"),
            DmToken::from(" "),
            DmToken::from("double"),
            DmToken::from("-"),
            DmToken::from("quote"),
            DmToken::from(" "),
            DmToken::from("\""),
            DmToken::from("\n"),
            DmToken::from("*/"),
            DmToken::from("\n"),
        ];

        let result = preprocesser.tokenize(&lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_preprocess_unmatched_quotes() {
        let mut preprocesser = DmPreProcessor::new();

        let lines = vec![
            "This is a test.",
            "#warn This shouldn't fail.",
            "#error Nor \"this",
        ];

        let expected = vec![
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("is"),
            DmToken::from(" "),
            DmToken::from("a"),
            DmToken::from(" "),
            DmToken::from("test"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("#"),
            DmToken::from("warn"),
            DmToken::from(" "),
            DmToken::from("This"),
            DmToken::from(" "),
            DmToken::from("shouldn"),
            DmToken::from("'"),
            DmToken::from("t"),
            DmToken::from(" "),
            DmToken::from("fail"),
            DmToken::from("."),
            DmToken::from("\n"),
            DmToken::from("#"),
            DmToken::from("error"),
            DmToken::from(" "),
            DmToken::from("Nor"),
            DmToken::from(" "),
            DmToken::from("\""),
            DmToken::from("this"),
            DmToken::from("\n"),
        ];

        let result = preprocesser.tokenize(&lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hard_lines() {
        let mut preprocesser = DmPreProcessor::new();

        let lines = vec![
        "\t\t\t\tparts += \"[FOURSPACES]First Death: <b>[ded[\"name\"]], [ded[\"role\"]], at [ded[\"area\"]]. Damage taken: [ded[\"damage\"]].[ded[\"last_words\"] ? \" Their last words were: \\\"[ded[\"last_words\"]]\\\"\" : \"\"]</b>\""
    ];

        let expected = vec![
            DmToken::from("\t\t\t\t"),
            DmToken::from("parts"),
            DmToken::from(" "),
            DmToken::from("+"),
            DmToken::from("="),
            DmToken::from(" "),
            DmToken::from("\""),
            DmToken::from("["),
            DmToken::from("FOURSPACES"),
            DmToken::from("]"),
            DmToken::from("First Death: <b>"),
            DmToken::from("["),
            DmToken::from("ded"),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("name"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("]"),
            DmToken::from(", "),
            DmToken::from("["),
            DmToken::from("ded"),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("role"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("]"),
            DmToken::from(", at "),
            DmToken::from("["),
            DmToken::from("ded"),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("area"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("]"),
            DmToken::from(". Damage taken: "),
            DmToken::from("["),
            DmToken::from("ded"),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("damage"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("]"),
            DmToken::from("."),
            DmToken::from("["),
            DmToken::from("ded"),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("last_words"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from(" "),
            DmToken::from("?"),
            DmToken::from(" "),
            DmToken::from("\""),
            DmToken::from(" Their last words were: \\\""),
            DmToken::from("["),
            DmToken::from("ded"),
            DmToken::from("["),
            DmToken::from("\""),
            DmToken::from("last_words"),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("]"),
            DmToken::from("\\\""),
            DmToken::from("\""),
            DmToken::from(" "),
            DmToken::from(":"),
            DmToken::from(" "),
            DmToken::from("\""),
            DmToken::from("\""),
            DmToken::from("]"),
            DmToken::from("</b>"),
            DmToken::from("\""),
            DmToken::from("\n"),
        ];

        let result = preprocesser.tokenize(&lines);
        assert_eq!(result, expected);
    }
}
