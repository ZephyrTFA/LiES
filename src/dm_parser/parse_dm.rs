use std::{io::Result, path::Path};

use super::DmParser;

impl DmParser<'_> {
    pub(super) fn parse_file_dm(
        &mut self,
        _path: &Path,
        #[allow(unused)] lines: &[&str],
    ) -> Result<()> {
        // TODO
        Ok(())
    }
}
