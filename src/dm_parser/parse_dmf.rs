use std::{io::Result, path::PathBuf};

use super::DmParser;

impl DmParser<'_> {
    pub(super) fn parse_file_dmf(&mut self, _path: &PathBuf, _lines: &[&str]) -> Result<()> {
        Ok(())
    }
}
