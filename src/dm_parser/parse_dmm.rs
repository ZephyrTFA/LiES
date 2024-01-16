use std::{io::Result, path::PathBuf};

use super::DmParser;

impl DmParser<'_> {
    pub(super) fn parse_file_dmm(&mut self, path: &PathBuf, _lines: &[&str]) -> Result<()> {
        println!("DMM file: `{}`", path.display());
        Ok(())
    }
}
