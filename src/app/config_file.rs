use std::{fs, path::PathBuf};

use color_eyre::owo_colors::OwoColorize;
use serde::Deserialize;

use crate::app::utils::TinoError;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TinoDirs {
    pub todos_dir: String,
    pub ideas_dir: String,
    pub notes_dir: String,
    pub academic_notes_dir: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ConfigFile {
    pub tino_dirs: TinoDirs,
}

impl ConfigFile {
    /// Pass `true` for testing.
    pub fn new(test: bool) -> Result<Self, TinoError> {
        let config_file_path: String;

        let mut test_config_dir = String::default();
        match dirs::home_dir() {
            Some(home_dir) => {
                test_config_dir = format!("{}/dev/tino/src/tests", home_dir.display());
            }
            None => {
                return Err(TinoError::HomeDirNotFound);
            }
        }

        if test {
            match Self::get_config_file_path(Some(PathBuf::from(test_config_dir))) {
                Err(error) => return Err(error),
                Ok(result) => config_file_path = result,
            }
        } else {
            match Self::get_config_file_path(dirs::config_dir()) {
                Err(error) => return Err(error),
                Ok(result) => config_file_path = result,
            }
        }

        let config_file_result = fs::read_to_string(config_file_path);

        match config_file_result {
            Ok(config_file_content) => match toml::from_str(&config_file_content) {
                Err(error) => Err(TinoError::DeserializeConfigFileContentFailed(error)),
                Ok(parsered_config_file) => Ok(parsered_config_file),
            },
            Err(error) => {
                eprintln!(
                    "{}",
                    "You need to create a .tino.toml file in you config directory."
                        .red()
                        .bold()
                );
                Err(TinoError::ReadConfigFileFailed(error))
            }
        }
    }

    fn get_config_file_path(config_dir_path: Option<PathBuf>) -> Result<String, TinoError> {
        match config_dir_path {
            Some(config_dir_path) => Ok(format!("{}/.tino.toml", config_dir_path.display())),
            None => Err(TinoError::ConfigDirNotFound),
        }
    }
}
