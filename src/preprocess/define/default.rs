use std::cell::RefCell;

use super::{definition::DefineDefinition, lib::DefineStore};

impl DefineStore {
    const STDDEF_DEFINES: RefCell<Vec<DefineDefinition>> = RefCell::new(vec![]);

    fn setup_defines() {
        let defines = Self::STDDEF_DEFINES;
        let mut defines = defines.borrow_mut();
        defines.append(&mut vec![
            DefineDefinition::new_define("DM_VERSION", vec!["515".into()]),
            DefineDefinition::new_define("DM_BUILD", vec!["1627".into()]),
        ]);
    }

    pub fn stddef_defines() -> Vec<DefineDefinition> {
        if Self::STDDEF_DEFINES.borrow().is_empty() {
            Self::setup_defines();
        }

        let defines = Self::STDDEF_DEFINES;
        return (*defines.borrow()).clone();
    }
}
