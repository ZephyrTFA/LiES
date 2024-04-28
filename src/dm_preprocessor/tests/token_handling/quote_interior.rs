#[test]
fn test_quote_interior() {
    use crate::dm_preprocessor::{DmPreProcessor, DmToken};

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
