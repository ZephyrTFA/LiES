use crate::dm_preprocessor::DmPreProcessor;

#[test]
fn test_special_symbols() {
    let symbols = [
        '"', '\'', '+', '-', '*', '/', '%', '^', '&', '|', '=', '<', '>', '!',
    ];
    for &symbol in &symbols {
        assert!(
            DmPreProcessor::should_start_new_token(symbol, "text"),
            "Failed at symbol: {}",
            symbol
        );
    }
}

#[test]
fn test_whitespace_transition() {
    assert!(DmPreProcessor::should_start_new_token(' ', "text"));
    assert!(!DmPreProcessor::should_start_new_token(' ', " "));
    assert!(!DmPreProcessor::should_start_new_token('\t', ""));
    assert!(DmPreProcessor::should_start_new_token(' ', "\t"));
}

#[test]
fn test_digit_transition() {
    assert!(DmPreProcessor::should_start_new_token('1', "text"));
    assert!(!DmPreProcessor::should_start_new_token('2', "123"));
    assert!(DmPreProcessor::should_start_new_token('a', "123"));
}

#[test]
fn test_no_transition() {
    assert!(!DmPreProcessor::should_start_new_token('a', "text"));
    assert!(!DmPreProcessor::should_start_new_token('3', "12"));
    assert!(DmPreProcessor::should_start_new_token('-', "-"));
}
