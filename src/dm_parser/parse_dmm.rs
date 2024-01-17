use std::{io::Result, path::Path};

use super::DmParser;

impl DmParser<'_> {
    pub(super) fn parse_file_dmm(&mut self, path: &Path, _lines: &[&str]) -> Result<()> {
        println!("DMM file: `{}`", path.display());
        Ok(())
    }
}
