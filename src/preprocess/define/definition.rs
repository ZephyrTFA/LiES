use crate::tokenize::token::Token;

#[derive(Default)]
pub struct DefineDefinition {
    is_macro: bool,
    name: String,
    body: Vec<Token>,
}

impl DefineDefinition {
    pub fn is_macro(&self) -> bool {
        self.is_macro
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &Vec<Token> {
        &self.body
    }

    pub fn new_define(name: &str, body: Vec<Token>) -> Self {
        let name = name.to_string();
        Self {
            is_macro: false,
            name,
            body,
        }
    }
}
