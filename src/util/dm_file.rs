use std::path::{Path, PathBuf};

use log::debug;

pub struct DmFile {
    path: PathBuf,
    lines: Vec<String>,
}

impl DmFile {
    pub fn new(environment_directory: &Path, path: impl Into<PathBuf>) -> Result<Self, String> {
        let path = path.into();
        let lines = Self::load_lines(&environment_directory.join(&path))?;
        Ok(Self { path, lines })
    }

    fn load_lines(path: &PathBuf) -> Result<Vec<String>, String> {
        let raw: String = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let mut lines = raw.lines();
        let lines: Vec<String> = lines.map(Self::sanitize_line).collect();
        Ok(lines)
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    fn sanitize_line(line: &str) -> String {
        line.replace('\r', "")
    }
}
