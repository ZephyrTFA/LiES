use std::{io::Result, path::Path};

use super::DmParser;

impl DmParser<'_> {
    pub(super) fn parse_file_dmf(&mut self, _path: &Path, _lines: &[&str]) -> Result<()> {
        Ok(())
    }
}
