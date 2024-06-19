use super::lib::tokenize_lines;

#[test]
fn tokenize_tests() {
    let input = [
        "100.000",
        "_alpha",
        "_22222_asda",
        "23123abf",
        "1231_1321",
        "",
        "// /***",
        "\t\t \t",
    ];
    let mut expected = [
        vec!["100.000"],
        vec!["_alpha"],
        vec!["_22222_asda"],
        vec!["23123", "abf"],
        vec!["1231", "_1321"],
        vec![],
        vec!["//", " ", "/***"],
        vec!["\t\t \t"],
    ];

    let actual = tokenize_lines(input.iter().cloned(), std::path::Path::new("test.test"))
        .expect("tokenize failed");
    let mut actual = actual.tokens().clone();
    actual.sort_by_key(|token| token.line());

    assert_eq!(
        expected.len() - 1,
        actual.last().unwrap().line(),
        "number of actual lines and expected lines differ"
    );

    let mut current = 0;
    for token in actual.iter() {
        if token.line() != current {
            assert!(
                expected[current].is_empty(),
                "still more tokens expected but line ended"
            );
            current = token.line();
        }
        assert!(
            !expected[current].is_empty(),
            "expected end of tokens for line but more tokens"
        );
        assert_eq!(
            expected[current].remove(0),
            token.value(),
            "expected different next token"
        );
    }
}
