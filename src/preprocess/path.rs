use std::path::{Path, PathBuf};

use super::PreprocessState;

impl PreprocessState {
    pub fn process_file_path(&self, path: &str) -> PathBuf {
        let env_dir = self.environment().working_directory();

        let file_dir;
        if let Some(define_file_dir) = self.environment().defines().get_define("FILE_DIR") {
            file_dir = define_file_dir
                .body()
                .iter()
                .map(|tok| tok.value().as_str())
                .collect::<Vec<_>>()
                .join("");
        } else {
            file_dir = ".".to_string();
        }

        let current_dir = self.environment().current_directory();
        let actual = Path::new(env_dir)
            .join(file_dir)
            .join(current_dir)
            .join(path);
        actual
    }
}
