#[test]
fn test_tokenize_comment_multline_commented_bad_end() {
    use crate::dm_preprocessor::{DmPreProcessor, DmToken};

    let mut preprocesser = DmPreProcessor::new();

    let lines = vec![
        "/*",
        " *",
        " * This is a comment.",
        " *",
        " // /*/",               // this doen't end the comment because the ending is broken
        "A lone single-quote '", // unmatched quotes will fail except in comments
        "A lone double-quote \"", // these won't fail because the commend doesn't end in one of the previous line
        "*/",
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
        DmToken::from("/*"),
        DmToken::from("/"),
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
        DmToken::from("*/"),
        DmToken::from("\n"),
    ];

    let result = preprocesser.tokenize(&lines);
    assert_eq!(result, expected);
}
