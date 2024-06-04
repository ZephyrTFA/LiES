use std::{collections::VecDeque, iter::Peekable, mem::take, rc::Rc};

pub mod scope;
pub mod type_path;

use log::debug;
use scope::Scope;

use crate::{tokens::dm_token::DmToken, util::ParseError};

use super::lib::DmParser;

impl DmParser {
    pub fn parse_tokens(&mut self, tokens: VecDeque<DmToken>) -> Result<(), ParseError> {
        let _scopes = self.parse_scopes(tokens)?;
        Ok(())
    }

    pub(super) fn parse_scopes(
        &mut self,
        tokens: VecDeque<DmToken>,
    ) -> Result<VecDeque<Rc<Scope>>, ParseError> {
        let mut tokens = tokens.into_iter().peekable();

        let mut scopes: VecDeque<Rc<Scope>> = VecDeque::default();
        let mut current_scope: Scope = Scope::default();

        let mut line_indent_set: bool = false;
        let mut line_indent_char_divisor: usize = 0;

        loop {
            let next_peek = tokens.peek();
            if next_peek.is_none() {
                break;
            } else if next_peek.is_some_and(|next| next.value() == "\n" && !next.is_in_string()) {
                line_indent_set = false;
                tokens.next();
                continue;
            }

            if !line_indent_set {
                let new_indent_level = Self::consume_indentation_level(&mut tokens)?;
                debug!("new indent level: {new_indent_level}");
                let line_indent_level;
                if line_indent_char_divisor != 0 {
                    line_indent_level = new_indent_level / line_indent_char_divisor;
                } else {
                    line_indent_char_divisor = new_indent_level;
                    line_indent_level = 1;
                }
                line_indent_set = true;

                if let Some(scope_indent_level) = current_scope.indentation_level() {
                    if scope_indent_level != line_indent_level {
                        let old_scope = Rc::new(take(&mut current_scope));
                        current_scope.set_parent(old_scope.clone())?;
                        scopes.push_back(old_scope);
                    }
                } else {
                    current_scope.set_indentation_level(line_indent_level);
                }

                continue;
            }

            if current_scope.effective_type_path().is_none() {
                current_scope.consume_type_path(&mut tokens)?;
                continue;
            }

            let _token = tokens
                .next()
                .expect("failed to get next token for scope parsing");
            debug!("{_token}");
        }

        Ok(scopes)
    }

    fn consume_indentation_level(
        tokens: &mut Peekable<impl Iterator<Item = DmToken>>,
    ) -> Result<usize, ParseError> {
        if tokens.peek().is_none() {
            return Err(ParseError::UNEXPECTED_EOL);
        }

        let mut indent_count = 0;
        while let Some(front) = tokens.peek() {
            debug!("{front:?}");
            if !front.value().chars().all(|c| matches!(c, ' ' | '\t')) {
                return Ok(indent_count);
            }
            indent_count += tokens.next().unwrap().value().len();
        }
        Ok(indent_count)
    }
}
