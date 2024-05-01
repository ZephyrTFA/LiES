use crate::tokens::dm_token::DmToken;

#[derive(Debug, Clone)]
pub struct DmDefineDefinition {
    name: String,
    body: Vec<DmToken>,
    macro_param_info: Option<MacroParamInfo>,
}

#[derive(Debug, Clone)]
pub struct MacroParamInfo {
    args: Vec<String>,
    arg_count: usize,
    last_arg_is_catch_all: bool,
}

impl MacroParamInfo {
    pub fn new(args: Vec<String>, arg_count: usize, last_arg_is_catch_all: bool) -> Self {
        Self {
            args,
            arg_count,
            last_arg_is_catch_all,
        }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn arg_count(&self) -> usize {
        self.arg_count
    }

    pub fn last_arg_is_catch_all(&self) -> bool {
        self.last_arg_is_catch_all
    }
}

impl DmDefineDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &[DmToken] {
        &self.body
    }

    pub fn is_macro(&self) -> bool {
        self.macro_param_info.is_some()
    }

    pub fn macro_param_info(&self) -> &MacroParamInfo {
        self.macro_param_info.as_ref().unwrap()
    }

    pub fn new_flag(name: &str) -> Self {
        Self {
            name: name.into(),
            body: vec![],
            macro_param_info: None,
        }
    }

    pub fn new_basic_replace(name: &str, body: &[DmToken]) -> Self {
        Self {
            name: name.into(),
            body: body.to_owned(),
            macro_param_info: None,
        }
    }

    pub fn new_macro(name: &str, body: Vec<DmToken>, macro_args: MacroParamInfo) -> Self {
        Self {
            name: name.into(),
            body,
            macro_param_info: Some(macro_args),
        }
    }
}
