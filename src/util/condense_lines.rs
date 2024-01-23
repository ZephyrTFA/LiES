// Condenses all lines that end with a backslash into a single line.
pub fn condense_lines(lines: &[impl Into<String> + Clone]) -> Vec<String> {
    if lines.is_empty() {
        return vec![];
    }

    let mut condensed = vec![];
    let mut current_line = String::new();

    for line in lines {
        let line = line.clone().into();
        if line.ends_with('\\') {
            current_line.push_str(&line[..line.len() - 1]);
        } else {
            current_line.push_str(&line);
            condensed.push(std::mem::take(&mut current_line));
        }
    }

    if !current_line.is_empty() {
        condensed.push(current_line);
    }

    condensed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn condense_lines_empty_input() {
        let lines: Vec<String> = vec![];
        let condensed = condense_lines(&lines);
        assert!(condensed.is_empty());
    }

    #[test]
    fn condense_lines_no_backslash() {
        let lines = vec!["line 1".to_string(), "line 2".to_string()];
        let condensed = condense_lines(&lines);
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
        let condensed = condense_lines(&lines);
        assert_eq!(
            condensed,
            vec!["line 1line 2".to_string(), "line 3line 4".to_string()]
        );
    }

    #[test]
    fn condense_lines_backslash_at_end() {
        let lines = vec!["line 1\\".to_string(), "line 2\\".to_string()];
        let condensed = condense_lines(&lines);
        assert_eq!(condensed, vec!["line 1line 2".to_string()]);
    }

    #[test]
    fn condense_lines_single_line_with_backslash() {
        let lines = vec!["line 1\\".to_string()];
        let condensed = condense_lines(&lines);
        assert_eq!(condensed, vec!["line 1".to_string()]);
    }

    #[test]
    fn condense_lines_single_line_without_backslash() {
        let lines = vec!["line 1".to_string()];
        let condensed = condense_lines(&lines);
        assert_eq!(condensed, lines);
    }
}
