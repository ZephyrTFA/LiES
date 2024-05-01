#[test]
fn test_tokenize_comment_multline_commented_bad_end() {
    use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken};

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
        DmToken::from("\n"),
        DmToken::from("\n"),
        DmToken::from("\n"),
        DmToken::from("\n"),
        DmToken::from("\n"),
        DmToken::from("\n"),
        DmToken::from("\n"),
        DmToken::from("\n"),
    ];

    let result = preprocesser.test_tokenize(&lines);
    assert_eq!(result, expected);
}
