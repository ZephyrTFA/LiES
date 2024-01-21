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
        DmToken::new("This".into()),
        DmToken::new(" ".into()),
        DmToken::new("is".into()),
        DmToken::new(" ".into()),
        DmToken::new("a".into()),
        DmToken::new(" ".into()),
        DmToken::new("test".into()),
        DmToken::new(".".into()),
        DmToken::new("\n".into()),
    ];

    let result = preprocessor.tokenize(&lines);

    assert_eq!(result, expected);
}

#[test]
fn test_tokenize_multiple_lines() {
    let mut preprocessor = DmPreProcessor::new();

    let lines: Vec<String> = vec!["This is a test.".into(), "Another test.".into()];

    let expected = vec![
        DmToken::new("This".into()),
        DmToken::new(" ".into()),
        DmToken::new("is".into()),
        DmToken::new(" ".into()),
        DmToken::new("a".into()),
        DmToken::new(" ".into()),
        DmToken::new("test".into()),
        DmToken::new(".".into()),
        DmToken::new("\n".into()),
        DmToken::new("Another".into()),
        DmToken::new(" ".into()),
        DmToken::new("test".into()),
        DmToken::new(".".into()),
        DmToken::new("\n".into()),
    ];

    let result = preprocessor.tokenize(&lines);

    assert_eq!(result, expected);
}

#[test]
fn test_tokenize_empty_lines() {
    let mut preprocessor = DmPreProcessor::new();

    let lines: Vec<String> = vec!["This is a test.".into(), "".into()];

    let expected: Vec<DmToken> = vec![
        DmToken::new("This".into()),
        DmToken::new(" ".into()),
        DmToken::new("is".into()),
        DmToken::new(" ".into()),
        DmToken::new("a".into()),
        DmToken::new(" ".into()),
        DmToken::new("test".into()),
        DmToken::new(".".into()),
        DmToken::new("\n".into()),
        DmToken::new("\n".into()),
    ];

    let result = preprocessor.tokenize(&lines);

    assert_eq!(result, expected);
}
