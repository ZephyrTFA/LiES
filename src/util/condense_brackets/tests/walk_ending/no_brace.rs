use std::panic;

#[test]
fn test_walk_ending_brace_no_brace() {
    use crate::util::condense_brackets::walk_ending::walk_to_next_ending_brace;

    let mut input = vec!["Line with no braces in it".into()];
    panic::set_hook(Box::new(|_| {}));
    panic::catch_unwind(move || walk_to_next_ending_brace(&mut input)).unwrap_err();
    _ = panic::take_hook();
}
