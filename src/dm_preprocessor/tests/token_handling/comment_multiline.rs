#[test]
fn test_tokenize_comment_multiline() {
    use crate::dm_preprocessor::{DmPreProcessor, DmToken};

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
