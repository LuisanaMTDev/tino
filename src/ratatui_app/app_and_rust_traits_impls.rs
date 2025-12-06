use ratatui::widgets::ListState;
use tui_input::Input;

use crate::app::config_file::ConfigFile;
/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub active_field: usize,
    pub config_file: ConfigFile,
    pub file_name_input: Input,
    pub type_items: Vec<String>,
    pub type_state: ListState,
    pub category_items: Vec<String>,
    pub category_state: ListState,
}

impl Default for App {
    fn default() -> Self {
        let mut type_state = ListState::default();
        type_state.select(Some(0));

        let mut category_state = ListState::default();
        category_state.select(Some(0));

        Self {
            running: false,
            file_name_input: Input::default(),
            active_field: 0,
            type_items: vec!["Todo".to_string(), "Ideas".to_string(), "Notes".to_string()],
            type_state,
            category_items: vec![
                "Project".to_string(),
                "Area".to_string(),
                "Resource".to_string(),
                "Archive".to_string(),
            ],
            category_state,
        }
    }
}
