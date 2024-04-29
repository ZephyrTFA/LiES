#[test]
fn test_string_interop_nested() {
    use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken};

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
