#[test]
fn test_tokenize_single_line() {
    use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken};

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
