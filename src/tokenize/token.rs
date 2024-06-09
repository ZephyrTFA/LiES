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
}

#[derive(Debug)]
pub struct FileTokens {
    file: String,
    tokens: Vec<Token>,
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
        let mut tokens: Vec<_> = tokens.collect();
        self.tokens.append(&mut tokens);
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn file(&self) -> &String {
        &self.file
    }
}
