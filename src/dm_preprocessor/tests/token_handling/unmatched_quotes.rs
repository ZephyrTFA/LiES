#[test]
fn test_tokenize_preprocess_unmatched_quotes() {
    use crate::dm_preprocessor::{DmPreProcessor, DmToken};

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
