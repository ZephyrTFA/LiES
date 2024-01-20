use std::path::{Path, PathBuf};

use log::debug;

use crate::{dm_preprocessor::DmPreProcessor, util::dm_file::DmFile};

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
        debug!(
            "Loading file `{}` from `{}`",
            wanted_path.display(),
            current_traversal.display(),
        );

        let load_from = self
            .environment_directory
            .join(self.preprocessor.get_base_file_dir());
        let wanted_path = load_from
            .join(current_traversal)
            .join(wanted_path)
            .canonicalize()
            .expect("failed to canonicalize wanted path");

        let actual_path = self.convert_canonical_path_to_relative(&wanted_path);
        debug!("Actual path: {}", actual_path.display());

        if let Some(parent) = actual_path.parent() {
            self.environment_traversal.push(parent.into());
        } else {
            return Err(format!(
                "Failed to determine parent offset for directory traversal in {}",
                line!()
            ));
        }

        let result = self.parse_file(&actual_path);
        if result.is_ok() {
            debug!("Successfully loaded file {}", actual_path.display());
        } else {
            debug!("Failed to load file {}", actual_path.display());
        }

        self.environment_traversal
            .pop() // not returning an Err here because this SHOULD not be possible
            .expect("failed to pop directory traversal?");
        result
    }

    fn parse_file(&mut self, file: impl Into<PathBuf>) -> Result<(), String> {
        let file = DmFile::new(&self.environment_directory, file.into())?;
        let tokens = self.preprocessor.preprocess(&file);
        for token in tokens {}
        Ok(())
    }
}
