use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::exit,
};

use log::{debug, error, info, trace, warn};

use crate::{
    dm_preprocessor::DmPreProcessor,
    util::{dm_file::DmFile, exit_codes::ERROR_CODE_DIRECTORY_TRAVERSAL_FAILED},
};

pub struct DmParser {
    preprocessor: DmPreProcessor,
    /// The order in which files were included. Uses a relative path from the environment directory.
    include_order: Vec<PathBuf>,
    environment_directory: PathBuf,
    environment_traversal: Vec<PathBuf>,
}

impl DmParser {
    pub fn new(environment_directory: impl Into<PathBuf>) -> Self {
        let environment_directory = environment_directory.into().canonicalize().unwrap();
        debug!(
            "DmParser new with env dir `{}`",
            environment_directory.display()
        );
        Self {
            preprocessor: DmPreProcessor::new(),
            include_order: vec![],
            environment_directory,
            environment_traversal: vec![],
        }
    }

    fn convert_canonical_path_to_relative(&self, path: &Path) -> PathBuf {
        path.strip_prefix(&self.environment_directory)
            .unwrap()
            .to_path_buf()
    }

    pub fn load(&mut self, file: impl Into<PathBuf>) -> Result<(), String> {
        let current_traversal = match self.environment_traversal.last() {
            Some(traversal) => traversal.clone(),
            None => ".".into(),
        };

        let wanted_path = file.into();
        if wanted_path.extension() == Some(OsStr::new("dmm")) {
            return Ok(());
        }

        info!("Parsing `{}`", wanted_path.display());
        let load_from = self
            .environment_directory
            .join(self.preprocessor.get_base_file_dir());
        let wanted_path = load_from
            .join(current_traversal)
            .join(wanted_path)
            .canonicalize()
            .expect("failed to canonicalize wanted path");

        let actual_path = self.convert_canonical_path_to_relative(&wanted_path);
        trace!("Actual path: {}", actual_path.display());

        if let Some(parent) = actual_path.parent() {
            self.environment_traversal.push(parent.into());
        } else {
            error!(
                "Failed to determine logical parent of `{}`",
                actual_path.display()
            );
            panic!();
        }

        let result = self.parse_file(&actual_path);
        if result.is_ok() {
            trace!("Successfully loaded file {}", actual_path.display());
        } else {
            trace!("Failed to load file {}", actual_path.display());
        }

        for pending_include in self.preprocessor.take_pending_includes() {
            self.load(pending_include)?;
        }

        self.environment_traversal
            .pop() // not returning an Err here because this SHOULD not be possible
            .expect("failed to pop directory traversal?");
        result
    }

    fn parse_file(&mut self, file: impl Into<PathBuf>) -> Result<(), String> {
        let file = DmFile::new(&self.environment_directory, file.into())?;
        let tokens = self.preprocessor.preprocess(&file);
        warn!("parsing not yet implemented.");
        for token in tokens {}
        Ok(())
    }
}
