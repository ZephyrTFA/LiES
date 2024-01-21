use crate::dm_preprocessor::DmPreProcessor;

#[test]
fn condense_lines_empty_input() {
    let lines: Vec<String> = vec![];
    let condensed = DmPreProcessor::condense_lines(&lines);
    assert!(condensed.is_empty());
}

#[test]
fn condense_lines_no_backslash() {
    let lines = vec!["line 1".to_string(), "line 2".to_string()];
    let condensed = DmPreProcessor::condense_lines(&lines);
    assert_eq!(condensed, lines);
}

#[test]
fn condense_lines_with_backslash() {
    let lines = vec![
        "line 1\\".to_string(),
        "line 2".to_string(),
        "line 3\\".to_string(),
        "line 4".to_string(),
    ];
    let condensed = DmPreProcessor::condense_lines(&lines);
    assert_eq!(
        condensed,
        vec!["line 1line 2".to_string(), "line 3line 4".to_string()]
    );
}

#[test]
fn condense_lines_backslash_at_end() {
    let lines = vec!["line 1\\".to_string(), "line 2\\".to_string()];
    let condensed = DmPreProcessor::condense_lines(&lines);
    assert_eq!(condensed, vec!["line 1line 2".to_string()]);
}

#[test]
fn condense_lines_single_line_with_backslash() {
    let lines = vec!["line 1\\".to_string()];
    let condensed = DmPreProcessor::condense_lines(&lines);
    assert_eq!(condensed, vec!["line 1".to_string()]);
}

#[test]
fn condense_lines_single_line_without_backslash() {
    let lines = vec!["line 1".to_string()];
    let condensed = DmPreProcessor::condense_lines(&lines);
    assert_eq!(condensed, lines);
}
