use ratatui::widgets::ListState;
use tui_input::Input;

use crate::app::config_file::ConfigFile;
/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub active_field: usize,
    pub open_editor: bool,
    pub config_file: ConfigFile,
    pub file_name_input: Input,
    pub type_items: Vec<String>,
    pub type_state: ListState,
    pub category_items: Vec<String>,
    pub category_state: ListState,
    pub tino_files: Vec<(String, String)>,
    pub tino_files_state: ListState,
}
