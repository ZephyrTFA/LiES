#[test]
fn test_walk_ending_brace_same_line() {
    use crate::util::condense_brackets::walk_ending::walk_to_next_ending_brace;

    let mut input = vec!["Line with {a brace} in it".into()];
    let output = walk_to_next_ending_brace(&mut input);
    assert_eq!(output, "Line with a brace in it");
    assert!(input.is_empty());
}
