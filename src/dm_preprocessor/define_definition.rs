use crate::tokens::dm_token::DmToken;

#[derive(Debug, Clone)]
pub struct DmDefineDefinition {
    name: String,
    body: Vec<DmToken>,
    macro_args: Option<Vec<String>>,
}

impl DmDefineDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &[DmToken] {
        &self.body
    }

    pub fn is_macro(&self) -> bool {
        self.macro_args.is_some()
    }

    pub fn macro_args(&self) -> &[String] {
        self.macro_args.as_ref().unwrap()
    }

    pub fn new_flag(name: &str) -> Self {
        Self {
            name: name.into(),
            body: vec![],
            macro_args: None,
        }
    }

    pub fn new_basic_replace(name: &str, body: &[DmToken]) -> Self {
        Self {
            name: name.into(),
            body: body.to_owned(),
            macro_args: None,
        }
    }

    pub fn new_macro(name: &str, body: Vec<DmToken>, macro_args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            body,
            macro_args: Some(macro_args),
        }
    }
}
