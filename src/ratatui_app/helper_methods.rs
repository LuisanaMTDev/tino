use std::{
    ffi::OsString,
    fs::{self, File},
};

use crate::{
    app::{config_file::ConfigFile, utils::TinoError},
    ratatui_app::types::{App, TinoFileTypes},
};
use chrono::Utc;
use softpath::prelude::*;

pub trait Helpers {
    fn generate_file_name(&mut self) -> Result<String, TinoError>;
    fn quit(&mut self);
    fn selected_type(&self) -> Option<&str>;
    fn selected_category(&self) -> Option<&str>;
    fn selected_tino_file(&self) -> Option<&str>;
    fn type_next(&mut self);
    fn type_previous(&mut self);
    fn category_next(&mut self);
    fn category_previous(&mut self);
    fn tino_file_next(&mut self);
    fn tino_file_previous(&mut self);
    fn get_tino_files(config_file: ConfigFile) -> anyhow::Result<Vec<(String, String)>>;
    fn get_tino_dir_files(
        tino_dir: String,
        tino_file_type: TinoFileTypes,
    ) -> anyhow::Result<Vec<(String, String)>>;
    fn format_tino_file(tino_file_type: TinoFileTypes, tino_file_name: OsString) -> String;
    fn get_file_content(&self) -> Result<String, TinoError>;
    fn create_tino_file(&mut self, tino_dir: &str) -> anyhow::Result<()>;
}

impl Helpers for App {
    fn generate_file_name(&mut self) -> Result<String, TinoError> {
        let user_input = self.file_name_input.value_and_reset().trim().to_string();
        let file_name: String;

        let now = Utc::now();
        let timestamp = now.format("%Y-%m-%dT%H:%M:%S").to_string();

        if user_input.is_empty() && self.selected_category() == Some("") {
            file_name = format!("{}.md", timestamp);
        } else if user_input.is_empty() {
            match self.selected_category() {
                Some(selected_category) => {
                    file_name = format!("{} - {}.md", timestamp, selected_category)
                }
                None => return Err(TinoError::NotSelectedCategory),
            }
        } else if self.selected_category() == Some("") {
            file_name = format!("{} {}.md", user_input, timestamp);
        } else {
            match self.selected_category() {
                Some(selected_category) => {
                    file_name = format!("{} {} - {}.md", user_input, timestamp, selected_category)
                }
                None => return Err(TinoError::NotSelectedCategory),
            }
        }

        Ok(file_name)
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn selected_type(&self) -> Option<&str> {
        self.type_state
            .selected()
            .map(|i| self.type_items[i].as_str())
    }

    fn selected_category(&self) -> Option<&str> {
        self.category_state
            .selected()
            .map(|i| self.category_items[i].as_str())
    }

    fn selected_tino_file(&self) -> Option<&str> {
        self.tino_files_state
            .selected()
            .map(|i| self.tino_files[i].1.as_str())
    }

    fn type_next(&mut self) {
        let i = match self.type_state.selected() {
            Some(i) => {
                if i >= self.type_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.type_state.select(Some(i));
    }

    fn type_previous(&mut self) {
        let i = match self.type_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.type_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.type_state.select(Some(i));
    }

    fn category_next(&mut self) {
        let i = match self.category_state.selected() {
            Some(i) => {
                if i >= self.category_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.category_state.select(Some(i));
    }

    fn category_previous(&mut self) {
        let i = match self.category_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.category_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.category_state.select(Some(i));
    }

    fn tino_file_next(&mut self) {
        let i = match self.tino_files_state.selected() {
            Some(i) => {
                if i >= self.tino_files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.tino_files_state.select(Some(i));
    }

    fn tino_file_previous(&mut self) {
        let i = match self.tino_files_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tino_files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.tino_files_state.select(Some(i));
    }

    fn get_tino_files(config_file: ConfigFile) -> anyhow::Result<Vec<(String, String)>> {
        let mut tino_files = vec![];

        tino_files.extend(Self::get_tino_dir_files(
            config_file.tino_dirs.todos_dir,
            TinoFileTypes::Todo,
        )?);
        tino_files.extend(Self::get_tino_dir_files(
            config_file.tino_dirs.ideas_dir,
            TinoFileTypes::Idea,
        )?);
        tino_files.extend(Self::get_tino_dir_files(
            config_file.tino_dirs.notes_dir,
            TinoFileTypes::Note,
        )?);
        tino_files.extend(Self::get_tino_dir_files(
            config_file.tino_dirs.academic_notes_dir,
            TinoFileTypes::AcademicNote,
        )?);

        Ok(tino_files)
    }

    fn format_tino_file(tino_file_type: TinoFileTypes, tino_file_name: OsString) -> String {
        match tino_file_type {
            TinoFileTypes::Todo => format!("TODO | {}", tino_file_name.display()),
            TinoFileTypes::Idea => format!("IDEA | {}", tino_file_name.display()),
            TinoFileTypes::Note => format!("NOTE | {}", tino_file_name.display()),
            TinoFileTypes::AcademicNote => format!("ACAD. NOTE | {}", tino_file_name.display()),
        }
    }
    fn get_file_content(&self) -> Result<String, TinoError> {
        match self.selected_tino_file() {
            Some(tino_file) => {
                match fs::read_to_string(tino_file) {
                    Ok(content) => return Ok(content),
                    Err(error) => return Err(TinoError::ReadTinoFileFailed(error)),
                };
            }
            None => Err(TinoError::NotSelectedTinoFile),
        }
    }
    fn create_tino_file(&mut self, tino_dir: &str) -> anyhow::Result<()> {
        let file_name = match self.generate_file_name() {
            Ok(generated_file_name) => generated_file_name,
            Err(error) => return Err(error.into()),
        };

        let bufpath = match tino_dir.into_path() {
            Ok(path) => match path.canonicalize() {
                Ok(extended_path) => extended_path.join(file_name),
                Err(error) => return Err(error.into()),
            },
            Err(error) => return Err(error.into()),
        };

        if let Err(error) = File::create(bufpath) {
            return Err(error.into());
        }
        self.tino_files = Self::get_tino_files(self.config_file.clone())?;
        Ok(())
    }

    fn get_tino_dir_files(
        tino_dir: String,
        tino_file_type: TinoFileTypes,
    ) -> anyhow::Result<Vec<(String, String)>> {
        let tino_todo_dir = fs::read_dir(tino_dir.into_path()?.canonicalize()?)?;
        let mut tino_files = vec![];
        for tino_file in tino_todo_dir {
            let tino_file = tino_file?;

            tino_files.push((
                Self::format_tino_file(tino_file_type, tino_file.file_name()),
                tino_file.path().canonicalize()?.display().to_string(),
            ));
        }
        Ok(tino_files)
    }
}
