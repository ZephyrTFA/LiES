use std::collections::HashMap;

use super::definition::DefineDefinition;

#[derive(Default)]
pub struct DefineStore {
    defines: HashMap<String, DefineDefinition>,
}

impl DefineStore {
    pub fn get_define(&self, key: &str) -> Option<&DefineDefinition> {
        self.defines.get(key)
    }

    pub fn insert_define(&mut self, define: DefineDefinition) {
        self.defines.insert(define.name().to_string(), define);
    }
}
