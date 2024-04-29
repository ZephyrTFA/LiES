#[test]
fn test_tokenize_empty() {
    use crate::dm_preprocessor::lib::DmPreProcessor;

    let mut preprocessor = DmPreProcessor::new();

    let lines: Vec<&str> = vec![];
    let expected = vec![];
    let result = preprocessor.test_tokenize(&lines);

    assert_eq!(result, expected);
}
