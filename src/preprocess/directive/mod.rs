use crate::util::parse_error::ParseError;

pub mod define;
pub mod r#if;
pub mod ifdef;
pub mod ifndef;
pub mod include;
pub mod lib;

type DirectiveResult = Result<(), ParseError>;
