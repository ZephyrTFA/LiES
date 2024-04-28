#[test]
fn test_string_interop() {
    use crate::dm_preprocessor::{DmPreProcessor, DmToken};

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
