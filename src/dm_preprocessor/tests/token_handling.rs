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
