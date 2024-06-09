use std::rc::Rc;

pub struct EnvironmentData {
    working_directory: String,
    include_order: Vec<IncludeOrderEntry>,
}

impl EnvironmentData {
    pub fn working_directory(&self) -> &String {
        &self.working_directory
    }

    pub fn include_order(&self) -> &Vec<IncludeOrderEntry> {
        &self.include_order
    }
}

pub struct IncludeOrderEntry {
    included_from: Option<Rc<IncludeOrderEntry>>,
    path: String,
}

impl IncludeOrderEntry {
    pub fn new(path: String) -> Self {
        Self {
            included_from: None,
            path,
        }
    }

    pub fn with_included_from(mut self, included_from: Rc<IncludeOrderEntry>) -> Self {
        self.included_from = Some(included_from);
        self
    }

    pub fn included_from(&self) -> Option<&Rc<IncludeOrderEntry>> {
        self.included_from.as_ref()
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}
