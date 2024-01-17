use std::{io::Result, path::PathBuf};

pub struct DmFile {
    path: PathBuf,
    lines: Vec<String>,
}

impl DmFile {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let lines = Self::load_lines(&path)?;
        Ok(Self { path, lines })
    }

    fn load_lines(path: &PathBuf) -> Result<Vec<String>> {
        let raw: String = std::fs::read_to_string(path)?;
        let lines: Vec<String> = raw.lines().map(Self::sanitize_line).collect();
        Ok(lines)
    }

    pub fn lines(&self) -> Vec<&str> {
        self.lines.iter().map(|line| line.as_str()).collect()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    fn sanitize_line(line: &str) -> String {
        line.trim().replace('\r', "")
    }
}
