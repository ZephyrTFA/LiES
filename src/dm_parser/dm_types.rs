use std::collections::HashMap;

pub struct DmPath {
    _parent_path: Option<String>,
    _vars: HashMap<String, DmVar>,
    _procs: HashMap<String, DmProc>,
}

pub struct DmVar {
    _name: String,
    _default_value: String,
    _value: String,
    _data: Option<String>,
}

pub struct DmProc {
    _name: String,
    _actions: Vec<DmAction>,
    _data: Option<String>,
}

pub struct DmAction {}
