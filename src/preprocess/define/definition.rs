use crate::tokenize::token::Token;

#[derive(Default)]
pub struct DefineDefinition {
    is_macro: bool,
    body: Vec<Token>,
}

impl DefineDefinition {
    pub fn is_macro(&self) -> bool {
        self.is_macro
    }

    pub fn body(&self) -> &Vec<Token> {
        &self.body
    }
}
