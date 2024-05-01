use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use log::{debug, error, info, trace};

#[cfg(not(debug_assertions))]
use log::warn;

use crate::{dm_preprocessor::lib::DmPreProcessor, util::dm_file::DmFile};

enum ParseLogMode {
    None,
    Directory,
    File,
}

impl std::str::FromStr for ParseLogMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "directory" | "dir" => Ok(Self::Directory),
            "file" => Ok(Self::File),
            _ => Err(format!("Unknown PARSE_LOG_MODE `{}`", s)),
        }
    }
}

pub struct DmParser {
    preprocessor: DmPreProcessor,
    /// The order in which files were included. Uses a relative path from the environment directory.
    _include_order: Vec<PathBuf>,
    environment_directory: PathBuf,
    parse_log_mode: ParseLogMode,
    parse_last_dir: PathBuf,
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
            _include_order: vec![],
            environment_directory,
            environment_traversal: vec![],
            parse_log_mode: env::var("LIES_PARSE_LOG_MODE")
                .unwrap_or_else(|_| "none".into())
                .parse()
                .expect("failed to parse LIES_PARSE_LOG_MODE"),
            parse_last_dir: ".".into(),
        }
    }

    fn convert_canonical_path_to_relative(&self, path: &Path) -> PathBuf {
        path.strip_prefix(&self.environment_directory)
            .unwrap()
            .to_path_buf()
    }

    pub fn load(&mut self, file: impl Into<PathBuf>) -> Result<(), String> {
        let current_traversal = self
            .environment_traversal
            .last()
            .cloned()
            .unwrap_or_else(|| ".".into());

        let wanted_path = file.into();

        if wanted_path.extension() == Some(OsStr::new("dmm")) {
            return Ok(());
        }

        let load_from = self
            .environment_directory
            .join(self.preprocessor.get_base_file_dir());

        let wanted_path = load_from.join(current_traversal).join(wanted_path);

        let wanted_path_str = wanted_path
            .to_str()
            .ok_or_else(|| "Failed to convert path to string".to_string())?;

        // Unix / docker fix
        let wanted_path_str_fixed = if cfg!(unix) {
            wanted_path_str.replace('\\', "/")
        } else {
            wanted_path_str.to_string()
        };
        let wanted_path = PathBuf::from(wanted_path_str_fixed);

        if !wanted_path.exists() {
            return Err(format!(
                "File or directory does not exist: {}",
                wanted_path.display()
            ));
        }
        let wanted_path = wanted_path
            .canonicalize()
            .map_err(|_| "Failed to canonicalize the wanted path".to_string())?;

        let actual_path = self.convert_canonical_path_to_relative(&wanted_path);

        // announce each directory we enter if the depth is lower than the set depth
        match self.parse_log_mode {
            ParseLogMode::Directory => {
                let current_dir = actual_path.parent().unwrap();
                if current_dir != self.parse_last_dir {
                    let name = current_dir.display().to_string();
                    info!(
                        "Parsing Directory: {}",
                        if name.is_empty() { "." } else { &name }
                    );
                    self.parse_last_dir = current_dir.into();
                }
            }
            ParseLogMode::File => {
                info!("Parsing File: `{}`", actual_path.display());
            }
            ParseLogMode::None => {}
        }

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

        self.preprocessor.add_to_include_order(&actual_path);
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
        #[cfg(not(debug_assertions))]
        warn!("parsing not yet implemented.");
        for _token in tokens {}
        Ok(())
    }
}
