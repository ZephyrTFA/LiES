use std::{io::Result, path::PathBuf};

use super::DmParser;

impl DmParser<'_> {
    pub(super) fn parse_file_dm(
        &mut self,
        _path: &PathBuf,
        #[allow(unused)] lines: &[&str],
    ) -> Result<()> {
        // TODO
        Ok(())
    }
}
