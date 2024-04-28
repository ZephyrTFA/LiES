#[test]
fn test_walk_ending_brace_strings_contain_brace() {
    use crate::util::condense_brackets::walk_ending::walk_to_next_ending_brace;

    let mut input = vec!["Line with {\"a {{ string {{ with {{ more braces\"} in it".into()];
    let output = walk_to_next_ending_brace(&mut input);
    assert_eq!(
        output,
        "Line with \"a {{ string {{ with {{ more braces\" in it"
    );
    assert!(input.is_empty());
}
