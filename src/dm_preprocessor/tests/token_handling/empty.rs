#[test]
fn test_tokenize_empty() {
    use crate::dm_preprocessor::DmPreProcessor;

    let mut preprocessor = DmPreProcessor::new();

    let lines: Vec<&str> = vec![];
    let expected = vec![];
    let result = preprocessor.tokenize(&lines);

    assert_eq!(result, expected);
}
