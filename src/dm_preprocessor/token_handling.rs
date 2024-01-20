use log::debug;

use super::DmPreProcessor;

#[derive(Debug, Clone)]
pub struct DmToken {
    value: String,
}

impl DmToken {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PartialEq for DmToken {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl DmPreProcessor {
    fn condense_lines(&mut self, lines: Vec<impl Into<String>>) -> Vec<String> {
        let mut lines = lines
            .into_iter()
            .map(|line| line.into())
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return lines;
        }

        let mut condensed = vec![];

        while !lines.is_empty() {
            let mut line = lines.remove(0);

            if line.ends_with('\\') {
                line.pop();
                line.push_str(&lines.remove(0));
            }

            condensed.push(line);
        }

        condensed
    }

    pub fn tokenize(&mut self, lines: Vec<impl Into<String>>) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = self.condense_lines(lines);
        let mut tokens: Vec<DmToken> = vec![];

        for line in condensed_lines {
            debug!("line: {}", line);
        }

        todo!();
    }

    fn should_keep_together(&mut self, left: &DmToken, right: &DmToken) -> bool {
        const OPERATOR_CHARS: &[char; 11] =
            &['+', '-', '*', '/', '%', '^', '&', '|', '~', '!', '='];

        let left = left.value();
        let right = right.value();
        if left.is_empty() && right.is_empty() {
            return true;
        }
        if left.is_empty() || right.is_empty() {
            return false;
        }

        if left.ends_with(OPERATOR_CHARS) && right.starts_with(OPERATOR_CHARS) {
            return true;
        }
        false
    }
}
