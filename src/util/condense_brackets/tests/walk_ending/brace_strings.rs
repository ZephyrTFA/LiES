#[test]
fn test_walk_ending_brace_strings() {
    use crate::util::condense_brackets::walk_ending::walk_to_next_ending_brace;

    let mut input = vec![
        "Line with {\"a string brace\"} in it".into(),
        "Line with {\"a string ".into(),
        "multi-line brace\"} in it".into(),
    ];
    let output = walk_to_next_ending_brace(&mut input);
    assert_eq!(output, "Line with \"a string brace\" in it");
    assert_eq!(
        input,
        vec![
            "Line with {\"a string ".to_string(),
            "multi-line brace\"} in it".to_string(),
        ]
    );

    let output = walk_to_next_ending_brace(&mut input);
    assert_eq!(output, "Line with \"a string multi-line brace\" in it");
    assert!(input.is_empty());
}
