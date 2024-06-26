use std::{
    env,
    path::{Path, PathBuf},
};

use log::{debug, error, info, trace, warn};

use crate::{
    dm_preprocessor::lib::DmPreProcessor,
    util::{dm_file::DmFile, ParseError},
};

enum ParseLogMode {
    None,
    Directory,
    File,
}

impl Default for ParseLogMode {
    fn default() -> Self {
        Self::None // byond default
    }
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

impl Default for DmParser {
    fn default() -> Self {
        Self::new(".")
    }
}

impl DmParser {
    pub fn environment_directory(&self) -> &PathBuf {
        &self.environment_directory
    }
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

    pub fn load_path(&mut self, path: impl Into<PathBuf>) -> Result<(), ParseError> {
        let current_traversal = self
            .environment_traversal
            .last()
            .cloned()
            .unwrap_or_else(|| ".".into());

        let load_from = self
            .environment_directory
            .join(self.preprocessor.get_base_file_dir());

        let wanted_path = load_from.join(current_traversal).join(path.into());
        let wanted_path_str = wanted_path.to_str().ok_or(
            ParseError::DM_FILE_LOAD_FAILURE
                .with_file_path(wanted_path.to_string_lossy().to_string()),
        )?;

        // Unix / docker fix
        let wanted_path_str_fixed = if cfg!(unix) {
            wanted_path_str.replace('\\', "/")
        } else {
            wanted_path_str.to_string()
        };
        let wanted_path = PathBuf::from(&wanted_path_str_fixed);

        if !wanted_path.exists() {
            return Err(ParseError::DM_FILE_LOAD_FAILURE.with_file_path(wanted_path_str_fixed));
        }
        let wanted_path = wanted_path.canonicalize().map_err(|_| {
            ParseError::PATH_CANONICALIZE_FAIL.with_file_path(wanted_path_str_fixed)
        })?;

        let actual_path = self.convert_canonical_path_to_relative(&wanted_path);
        self.load_file(DmFile::new(&self.environment_directory, actual_path)?)
    }

    pub fn load_file(&mut self, file: DmFile) -> Result<(), ParseError> {
        let actual_path = file.path();

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

        self.preprocessor.add_to_include_order(actual_path);
        let result = self.parse_file(&file);
        if result.is_ok() {
            trace!("Successfully loaded file {}", actual_path.display());
        } else {
            trace!("Failed to load file {}", actual_path.display());
        }

        for pending_include in self.preprocessor.take_pending_includes() {
            self.load_path(pending_include)?;
        }

        self.environment_traversal
            .pop() // not returning an Err here because this SHOULD not be possible
            .expect("failed to pop directory traversal?");

        result.map_err(|err| err.with_file_path(file.path().to_str().unwrap().to_string()))
    }

    fn parse_file(&mut self, file: &DmFile) -> Result<(), ParseError> {
        let file_extension = file
            .path()
            .extension()
            .expect("parsing file without extension!")
            .to_str()
            .expect("failed to convert extension to str");
        if !matches!(file_extension, "dme" | "dm") {
            warn!("Skipping File: {}", file.path().display());
            return Ok(());
        }

        let tokens = self.preprocessor.preprocess(file)?;
        self.parse_tokens(tokens)
    }
}
