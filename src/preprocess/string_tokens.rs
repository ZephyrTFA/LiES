use crate::tokenize::token::Token;

pub struct StringLiteral {
    _id: usize,
    _tokens: Vec<Token>,
}

impl StringLiteral {
    pub fn new(id: usize, tokens: Vec<Token>) -> Self {
        Self {
            _id: id,
            _tokens: tokens,
        }
    }
}
