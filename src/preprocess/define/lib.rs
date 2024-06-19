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

    pub fn set_define(&mut self, key: &str, value: DefineDefinition) {
        self.defines.insert(key.to_string(), value);
    }
}
