#[test]
fn test_walk_ending_brace_multiple_lines() {
    use crate::util::condense_brackets::walk_ending::walk_to_next_ending_brace;

    let mut input = vec![
        "Line with {a brace ".into(),
        "in it ".into(),
        "and another brace} in it".into(),
        "And a line without".into(),
    ];
    let output = walk_to_next_ending_brace(&mut input);
    assert_eq!(output, "Line with a brace in it and another brace in it");
    assert_eq!(input, vec!["And a line without".to_string()]);
}
