use std::{collections::VecDeque, path::Path, rc::Rc};

use log::debug;

use super::define::lib::DefineStore;

pub struct EnvironmentData {
    working_directory: String,
    include_order: Vec<Rc<IncludeOrderEntry>>,
    defines: DefineStore,
    current_file_queue: VecDeque<String>,
}

impl EnvironmentData {
    pub fn working_directory(&self) -> &str {
        &self.working_directory
    }

    pub fn include_order(&self) -> &Vec<Rc<IncludeOrderEntry>> {
        &self.include_order
    }

    pub fn new(working_directory: String) -> Self {
        Self {
            working_directory,
            include_order: vec![],
            defines: DefineStore::default(),
            current_file_queue: VecDeque::default(),
        }
    }

    pub fn add_include(&mut self, include: Rc<IncludeOrderEntry>) {
        self.include_order.push(include)
    }

    pub fn current_file(&self) -> Option<&str> {
        self.current_file_queue.back().map(|string| string.as_str())
    }

    pub fn defines(&self) -> &DefineStore {
        &self.defines
    }

    pub fn defines_mut(&mut self) -> &mut DefineStore {
        &mut self.defines
    }

    pub fn push_current_file(&mut self, file: &str) {
        self.current_file_queue.push_back(file.to_string());
        debug!("PuCF: {file}");
    }

    pub fn pop_current_file(&mut self) {
        self.current_file_queue
            .pop_back()
            .expect("pop current file without active file");
        debug!("PoCF");
    }

    pub fn current_directory(&self) -> &str {
        if let Some(current_file) = self.current_file() {
            let as_path = Path::new(current_file);
            let as_path = as_path.parent().unwrap();
            return as_path.to_str().unwrap();
        }
        "."
    }
}

pub struct IncludeOrderEntry {
    included_from: Option<Rc<IncludeOrderEntry>>,
    path: String,
}

impl IncludeOrderEntry {
    pub fn new(path: &str) -> Self {
        Self {
            included_from: None,
            path: path.to_string(),
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
