use crate::util::parse_error::ParseError;

pub mod include;
pub mod lib;

type DirectiveResult = Result<(), ParseError>;
