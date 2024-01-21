use std::collections::HashMap;

pub struct DmDefineDefinition {
    name: String,
    body: String,
    is_macro: bool,
    macro_args: Vec<String>,
}

impl DmDefineDefinition {
    pub fn insert_into_map(self, map: &mut HashMap<String, DmDefineDefinition>) -> &Self {
        let name = self.name.clone();
        map.insert(self.name.clone(), self);
        map.get(&name).unwrap()
    }

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

    pub fn new_flag(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            body: String::new(),
            is_macro: false,
            macro_args: vec![],
        }
    }

    pub fn new_basic_replace(name: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            body: body.into(),
            is_macro: false,
            macro_args: vec![],
        }
    }

    pub fn new_macro(
        name: impl Into<String>,
        macro_args: Vec<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            body: body.into(),
            is_macro: true,
            macro_args,
        }
    }
}
