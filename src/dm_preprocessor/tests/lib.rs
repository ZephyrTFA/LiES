use crate::tokens::dm_token::DmToken;

#[test]
fn test_define_replacement() {
    use crate::dm_preprocessor::{define_definition::DmDefineDefinition, lib::DmPreProcessor};

    let mut preprocess = DmPreProcessor::new();
    preprocess.add_define(DmDefineDefinition::new_basic_replace(
        "TEST1",
        &["test1".into()],
    ));
    preprocess.add_define(DmDefineDefinition::new_basic_replace(
        "TEST2",
        &["test2".into(), " ".into(), "test2".into()],
    ));
    preprocess.add_define(DmDefineDefinition::new_basic_replace(
        "TEST3",
        &[
            "test3".into(),
            " ".into(),
            "test3".into(),
            " ".into(),
            "test3".into(),
        ],
    ));

    let input: Vec<DmToken> = vec!["TEST1".into(), "TEST2".into(), "TEST3".into()];
    let expected: Vec<DmToken> = vec![
        "test1".into(),
        "test2".into(),
        " ".into(),
        "test2".into(),
        "test3".into(),
        " ".into(),
        "test3".into(),
        " ".into(),
        "test3".into(),
    ];

    let mut tokens = input;
    let mut output = vec![];

    while !tokens.is_empty() {
        let token = tokens.remove(0);
        let token = preprocess.do_define_replacement(token, &mut tokens);
        if let Some(token) = token {
            output.push(token);
        }
    }

    assert_eq!(output, expected);
}
