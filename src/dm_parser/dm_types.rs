use std::collections::HashMap;

pub struct DmPath {
    parent_path: Option<String>,
    vars: HashMap<String, DmVar>,
    procs: HashMap<String, DmProc>,
}

pub struct DmVar {
    name: String,
    default_value: String,
    value: String,
    data: Option<String>,
}

pub struct DmProc {
    name: String,
    actions: Vec<DmAction>,
    data: Option<String>,
}

pub struct DmAction {}
