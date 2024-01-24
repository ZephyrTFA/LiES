/** Count the number of backslashes. */
pub fn count_backslashes(string: &str) -> usize {
    let mut count = 0;
    for char in string.chars().rev() {
        if char == '\\' {
            count += 1;
        } else {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_backslashes_empty() {
        let backslashes = "";

        assert_eq!(count_backslashes(backslashes), 0);
    }

    #[test]
    fn test_count_backslashes() {
        let backslashes = "\\\\\\";

        assert_eq!(count_backslashes(backslashes), 3);
    }
}
