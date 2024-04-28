use super::walk_ending::walk_to_next_ending_brace;

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
