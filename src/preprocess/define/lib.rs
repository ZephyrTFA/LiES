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

    pub(crate) fn remove_define(&mut self, define_name: &str) -> Option<DefineDefinition> {
        self.defines.remove(define_name)
    }
}

pub fn stddef_defines() -> Vec<DefineDefinition> {
    vec![
        DefineDefinition::new_define("DM_VERSION", vec!["515".into()]),
        DefineDefinition::new_define("DM_BUILD", vec!["1627".into()]),
        DefineDefinition::new_define("TRUE", vec!["1".into()]),
        DefineDefinition::new_define("FALSE", vec!["0".into()]),
    ]
}
