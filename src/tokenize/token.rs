use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    value: String,
    line: usize,
    column: usize,
}

impl Token {
    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn new(value: String, line: usize, column: usize) -> Self {
        Self {
            value,
            line,
            column,
        }
    }

    pub fn is_only_spacing(&self) -> bool {
        self.value.chars().all(|c| matches!(c, ' ' | '\t'))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct FileTokens {
    file: String,
    tokens: Vec<Token>,
}

impl<'a> From<&'a FileTokens> for Vec<Vec<&'a Token>> {
    fn from(value: &'a FileTokens) -> Self {
        value.tokens_by_line()
    }
}

impl<'a> From<&'a FileTokens> for Vec<&'a Token> {
    fn from(value: &'a FileTokens) -> Self {
        value.tokens.iter().collect()
    }
}

impl FileTokens {
    pub fn new(file: String) -> Self {
        Self {
            file,
            tokens: vec![],
        }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn add_tokens(&mut self, tokens: impl Iterator<Item = Token>) {
        self.tokens.append(&mut tokens.collect());
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn tokens_by_line(&self) -> Vec<Vec<&Token>> {
        let tokens = self.tokens();

        let mut tokens_by_line = vec![];
        for _ in 0..tokens.last().unwrap().line + 1 {
            tokens_by_line.push(vec![]);
        }
        for token in tokens {
            tokens_by_line[token.line].push(token);
        }
        tokens_by_line
    }

    pub fn file(&self) -> &String {
        &self.file
    }
}
