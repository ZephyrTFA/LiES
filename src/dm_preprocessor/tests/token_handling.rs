use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

#[test]
fn test_tokenize_empty() {
    let mut preprocessor = DmPreProcessor::new();

    let lines = vec![];
    let expected = vec![];
    let result = preprocessor.tokenize(&lines);

    assert_eq!(result, expected);
}

#[test]
fn test_tokenize_single_line() {
    let mut preprocessor = DmPreProcessor::new();

    let lines: Vec<String> = vec!["This is a test.".into()];

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

    let lines: Vec<String> = vec!["This is a test.".into(), "Another test.".into()];

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

    let lines: Vec<String> = vec!["This is a test.".into(), "".into()];

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
fn test_condense_lines() {
    let mut preprocessor = DmPreProcessor::new();

    let lines: Vec<String> = vec!["This is a test. \\".into(), "Another test.".into()];

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
fn test_tokenize_comment() {
    let mut preprocessor = DmPreProcessor::new();
    let lines = vec![
        "This is a test.".into(),
        "// This is a comment.".into(),
        "Another test.".into(),
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
        "This is a test.".into(),
        "/*".into(),
        "This is a comment.".into(),
        "*/".into(),
        "Another test.".into(),
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

fn test_tokenize_comment_multline_commented_bad_end() {
    let mut preprocesser = DmPreProcessor::new();

    let lines = vec![
        "/*".into(),
        " *".into(),
        " * This is a comment.".into(),
        " *".into(),
        " // /*/".into(), // this doen't end the comment because the ending is broken
        "A lone single-quote '".into(), // unmatched quotes will fail except in comments
        "A lone double-quote \"".into(), // these won't fail because the commend doesn't end in one of the previous line
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
        DmToken::from("/*/"),
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
    ];

    let result = preprocesser.tokenize(&lines);
    assert_eq!(result, expected);
}

fn test_tokenize_preprocess_unmatched_quotes() {
    let mut preprocesser = DmPreProcessor::new();
    let lines = vec![
        "This is a test.".into(),
        "#warn This shouldn't fail.".into(),
        "#error Nor \"this".into(),
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
