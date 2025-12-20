use ratatui::widgets::ListState;
use tui_input::Input;

use crate::app::config_file::ConfigFile;

#[derive(Clone, Copy)]
pub enum TinoFileTypes {
    Todo,
    Idea,
    Note,
    AcademicNote,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub active_field: usize,
    pub open_editor: bool,
    pub config_file: ConfigFile,
    pub scroll_position: (u16, u16),
    pub file_name_input: Input,
    pub type_items: Vec<String>,
    pub type_state: ListState,
    pub category_items: Vec<String>,
    pub category_state: ListState,
    pub tino_files: Vec<(String, String)>,
    pub tino_files_state: ListState,
    pub file_to_preview: String,
}
