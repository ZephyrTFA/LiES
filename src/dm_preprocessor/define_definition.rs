pub struct DmDefineDefinition {
    name: String,
    body: String,
    is_macro: bool,
    macro_args: Vec<String>,
}

impl DmDefineDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn is_macro(&self) -> bool {
        self.is_macro
    }

    pub fn macro_args(&self) -> &[String] {
        &self.macro_args
    }

    pub fn new_flag(name: String) -> Self {
        Self {
            name,
            body: String::new(),
            is_macro: false,
            macro_args: vec![],
        }
    }

    pub fn new_basic_replace(name: String, body: String) -> Self {
        Self {
            name,
            body,
            is_macro: false,
            macro_args: vec![],
        }
    }

    pub fn new_macro(name: String, macro_args: Vec<String>, body: String) -> Self {
        Self {
            name,
            body,
            is_macro: true,
            macro_args,
        }
    }
}
