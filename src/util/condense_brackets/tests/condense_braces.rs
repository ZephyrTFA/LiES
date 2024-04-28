#[test]
fn test_condense_braces() {
    use crate::util::condense_brackets::condense::condense_braces;

    let input = vec![
        "Line with no braces in it",
        "Line with {a brace} in it",
        "Line with {a brace ",
        "in it ",
        "and another brace} in it",
        "And another line without",
        "And now a string with a brace in it: @{\"a string",
        " brace\"} in it",
        "{\"\"\"}",
    ];
    let output = condense_braces(&input);
    assert_eq!(
        output,
        vec![
            "Line with no braces in it".to_string(),
            "Line with a brace in it".to_string(),
            "Line with a brace in it and another brace in it".to_string(),
            "And another line without".to_string(),
            "And now a string with a brace in it: @\"a string brace\" in it".to_string(),
            "\"\\\"\"".to_string(),
        ]
    );
}
