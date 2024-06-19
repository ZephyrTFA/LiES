use crate::util::parse_error::ParseError;

pub mod define;
pub mod include;
pub mod lib;

type DirectiveResult = Result<(), ParseError>;
