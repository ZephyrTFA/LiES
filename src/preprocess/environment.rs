use std::{collections::VecDeque, path::Path, rc::Rc};

use super::define::lib::{stddef_defines, DefineStore};

pub struct EnvironmentData {
    working_directory: String,
    include_order: Vec<Rc<IncludeOrderEntry>>,
    defines: DefineStore,
    current_file_queue: VecDeque<CurrentFileEntry>,
}

pub struct CurrentFileEntry {
    #[allow(dead_code)]
    path: String,
    full_path: String,
}
impl CurrentFileEntry {
    pub fn new(path: &str, full_path: &str) -> Self {
        Self {
            path: path.to_string(),
            full_path: full_path.to_string(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn full_path(&self) -> &str {
        &self.full_path
    }
}

impl EnvironmentData {
    pub fn working_directory(&self) -> &str {
        &self.working_directory
    }

    pub fn include_order(&self) -> &Vec<Rc<IncludeOrderEntry>> {
        &self.include_order
    }

    pub fn new(working_directory: String, include_stddef: bool) -> Self {
        let mut defines = DefineStore::default();
        if include_stddef {
            for define in stddef_defines() {
                defines.insert_define(define);
            }
        }

        Self {
            working_directory,
            include_order: vec![],
            defines,
            current_file_queue: VecDeque::default(),
        }
    }

    pub fn add_include(&mut self, include: Rc<IncludeOrderEntry>) {
        self.include_order.push(include)
    }

    pub fn current_file(&self) -> Option<&CurrentFileEntry> {
        self.current_file_queue.back()
    }

    pub fn defines(&self) -> &DefineStore {
        &self.defines
    }

    pub fn defines_mut(&mut self) -> &mut DefineStore {
        &mut self.defines
    }

    pub fn push_current_file(&mut self, entry: CurrentFileEntry) {
        self.current_file_queue.push_back(entry);
    }

    pub fn pop_current_file(&mut self) {
        self.current_file_queue
            .pop_back()
            .expect("pop current file without active file");
    }

    pub fn current_directory(&self) -> Option<&str> {
        self.current_file().map(|file| {
            Path::new(file.full_path.as_str())
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        })
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
