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
