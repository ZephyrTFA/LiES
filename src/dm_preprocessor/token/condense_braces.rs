use ::log::{error, trace};
use std::panic;

/// Iterates through a slice of string like objects and condenses curly braces into a single line.
/// Must be done after condense_lines.
pub fn condense_braces(lines: &[impl Into<String> + Clone]) -> Vec<String> {
    let mut lines: Vec<String> = lines.iter().map(|line| line.clone().into()).collect();

    let mut condensed = vec![];
    while !lines.is_empty() {
        let line = lines.first().unwrap();
        if !line.contains('{') {
            condensed.push(lines.remove(0));
            continue;
        }
        condensed.push(walk_to_next_ending_brace(&mut lines));
    }

    condensed
}

fn walk_to_next_ending_brace(lines: &mut Vec<String>) -> String {
    if lines.is_empty() {
        error!("No lines to walk through");
        panic!()
    }

    if lines.first().is_some_and(|line| !line.contains('{')) {
        error!("No braces in first line");
        panic!()
    }

    let mut final_line = String::new();
    let mut braces = 0;
    let mut first_brace_spliced = false;
    let mut first_brace: isize = -1;
    let mut quoted = false;
    while !lines.is_empty() {
        if braces == 0 {
            quoted = false;
        }

        let mut escaped = false;
        let line = lines.remove(0);
        trace!("Line: `{}`", line);
        for (char_idx, char) in line.chars().enumerate() {
            trace!("char: `{}`", char);
            match char {
                '{' if !escaped && !quoted => {
                    braces += 1;
                    trace!("walk_ending_brace: braces = {braces}");
                    if braces == 1 {
                        first_brace = char_idx as isize
                    };
                }
                '}' if !escaped && !quoted => {
                    braces -= 1;
                    trace!("walk_ending_brace: braces = {braces}");
                    if braces == 0 {
                        if !first_brace_spliced {
                            final_line.push_str(&line[..first_brace as usize]); // copy before the first brace
                            final_line.push_str(&line[first_brace as usize + 1..char_idx]); // copy between the braces
                            final_line.push_str(&line[char_idx + 1..]); // copy after the last brace
                        } else {
                            final_line.push_str(&line[..char_idx]); // copy before the ending brace
                            final_line.push_str(&line[char_idx + 1..]); // copy after the ending brace
                        }
                        return final_line;
                    }
                }
                '\\' => {
                    escaped = !escaped;
                    trace!("walk_ending_brace: escaped = {escaped}");
                }
                '"' if !escaped => {
                    quoted = !quoted;
                    trace!("walk_ending_brace: quoted = {quoted}");
                }
                _ if escaped => {
                    escaped = false;
                    trace!("walk_ending_brace: escaped = {escaped}");
                }
                _ => {}
            }
        }
        if !first_brace_spliced && first_brace != -1 {
            final_line.push_str(&line[..first_brace as usize]);
            final_line.push_str(&line[first_brace as usize + 1..]);
            first_brace_spliced = true;
        } else {
            final_line.push_str(&line);
        }
    }
    final_line
}

#[test]
fn test_condense_braces() {
    let input = vec![
        "Line with no braces in it",
        "Line with {a brace} in it",
        "Line with {a brace ",
        "in it ",
        "and another brace} in it",
        "And another line without",
        "And now a string with a brace in it: @{\"a string",
        " brace\"} in it",
    ];
    let output = condense_braces(&input);
    assert_eq!(
        output,
        vec![
            "Line with no braces in it".to_string(),
            "Line with a brace in it".to_string(),
            "Line with a brace in it and another brace in it".to_string(),
            "And another line without".to_string(),
            "And now a string with a brace in it: @\"a string brace\" in it".to_string(),
        ]
    );
}

#[test]
fn test_walk_ending_brace_no_brace() {
    let mut input = vec!["Line with no braces in it".into()];
    panic::set_hook(Box::new(|_| {}));
    panic::catch_unwind(move || walk_to_next_ending_brace(&mut input)).unwrap_err();
    _ = panic::take_hook();
}

#[test]
fn test_walk_ending_brace_same_line() {
    let mut input = vec!["Line with {a brace} in it".into()];
    let output = walk_to_next_ending_brace(&mut input);
    assert_eq!(output, "Line with a brace in it");
    assert!(input.is_empty());
}

#[test]
fn test_walk_ending_brace_multiple_lines() {
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

#[test]
fn test_walk_ending_brace_strings() {
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

#[test]
fn test_walk_ending_brace_strings_contain_brace() {
    let mut input = vec!["Line with {\"a {{ string {{ with {{ more braces\"} in it".into()];
    let output = walk_to_next_ending_brace(&mut input);
    assert_eq!(
        output,
        "Line with \"a {{ string {{ with {{ more braces\" in it"
    );
    assert!(input.is_empty());
}
