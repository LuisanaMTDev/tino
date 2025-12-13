use crate::ratatui_app::app_and_rust_traits_impls::App;
use chrono::Utc;

pub trait Helpers {
    fn generate_file_name(&mut self) -> String;
    fn quit(&mut self);
    fn selected_type(&self) -> Option<&str>;
    fn selected_category(&self) -> Option<&str>;
    fn type_next(&mut self);
    fn type_previous(&mut self);
    fn category_next(&mut self);
    fn category_previous(&mut self);
}

impl Helpers for App {
    fn generate_file_name(&mut self) -> String {
        let user_input = self.file_name_input.value_and_reset();
        let file_name: String;

        let now = Utc::now();
        let timestamp = now.format("%Y-%m-%dT%H:%M:%S").to_string();

        if user_input.is_empty() && self.selected_category() == Some("") {
            file_name = format!("{}.md", timestamp);
        } else if user_input.is_empty() {
            file_name = format!("{} - {}.md", timestamp, self.selected_category().unwrap());
        } else if self.selected_category() == Some("") {
            file_name = format!("{} {}.md", user_input, timestamp);
        } else {
            file_name = format!(
                "{} {} - {}.md",
                user_input,
                timestamp,
                self.selected_category().unwrap()
            );
        }

        file_name
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
}
