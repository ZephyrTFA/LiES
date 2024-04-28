#[test]
fn test_tokenize_empty_lines() {
    use crate::dm_preprocessor::{DmPreProcessor, DmToken};

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
