use std::{
    collections::HashMap,
    io::{Error, Result},
    path::PathBuf,
};

use self::{dm_file::DmFile, preprocessor::PreprocessState};

mod define_definition;
pub mod dm_file;
mod parse_dm;
mod parse_dmf;
mod parse_dmm;
mod preprocessor;
mod preprocessor_if;
mod token_store;

pub struct DmParser<'a> {
    files: HashMap<PathBuf, DmFile>,
    directory_traversal: Vec<PathBuf>,
    include_order: Vec<PathBuf>,
    work_directory: PathBuf,
    preprocess_state: PreprocessState<'a>,
}

impl DmParser<'_> {
    pub fn new(work_directory: impl Into<PathBuf>) -> Self {
        let work_directory = work_directory.into();
        Self {
            include_order: vec![],
            files: HashMap::new(),
            work_directory,
            preprocess_state: PreprocessState::new(),
            directory_traversal: vec![],
        }
    }

    pub fn load(&mut self, path: &str) -> Result<()> {
        let final_path = self
            .work_directory
            .join(self.preprocess_state.base_file_dir())
            .join(
                self.directory_traversal
                    .last()
                    .unwrap_or(&PathBuf::from(".")),
            )
            .join(path);
        println!("Loading File: `{}`", final_path.display());
        self.load_file(DmFile::new(final_path)?)
    }

    fn load_file(&mut self, file: DmFile) -> Result<()> {
        let path = file.path().clone();

        if self.files.contains_key(&path) {
            eprintln!("Attempted to parse a file twice: `{}`", path.display());
            return Ok(());
        }

        let lines: Vec<String> = file
            .lines()
            .iter()
            .map(|line| line.to_string().clone())
            .collect();

        self.include_order.push(path.clone());
        self.directory_traversal.push(
            path.parent()
                .ok_or(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "failed to find parent directory",
                ))?
                .to_path_buf(),
        );

        self.files.insert(path.clone(), file);
        let extension = path
            .extension()
            .ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "failed to find file extension",
            ))?
            .to_str()
            .ok_or(Error::new(
                std::io::ErrorKind::InvalidData,
                "failed to parse file extension",
            ))?;

        let lines = self.preprocess(&path, lines)?;
        let lines: Vec<&str> = lines.iter().map(|line| line.as_str()).collect();

        let result = match extension {
            "dme" => Ok(()), // preprocess only
            "dmf" => self.parse_file_dmf(&path, lines.as_slice()),
            "dmm" => self.parse_file_dmm(&path, lines.as_slice()),
            "dm" => self.parse_file_dm(&path, lines.as_slice()),
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid file type",
                ))
            }
        };
        self.directory_traversal.pop();
        result
    }
}
