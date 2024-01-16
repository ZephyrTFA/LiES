use std::collections::HashMap;

pub struct DefineDefinition {
    pub name: String,
    pub body: Option<String>,
    pub is_macro: bool,
    pub args: Option<Vec<String>>,
}

impl DefineDefinition {
    pub fn insert_into_map(self, map: &mut HashMap<String, DefineDefinition>) {
        map.insert(self.name.to_string(), self);
    }

    pub fn new_flag(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            body: None,
            is_macro: false,
            args: None,
        }
    }

    pub fn new_basic_replace(name: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            body: Some(body.into()),
            is_macro: false,
            args: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_macro_replace(
        name: impl Into<String>,
        args: Vec<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            body: Some(body.into()),
            is_macro: true,
            args: Some(args),
        }
    }
}
