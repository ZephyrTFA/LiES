use ::log::{error, trace};

pub fn walk_to_next_ending_brace(lines: &mut Vec<String>) -> String {
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
