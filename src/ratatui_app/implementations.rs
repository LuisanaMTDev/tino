use std::fs::File;
use std::path::PathBuf;

use crate::app::config_file::ConfigFile;
use crate::ratatui_app::app_and_rust_traits_impls::App;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use softpath::prelude::*;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(config_file: ConfigFile) -> Self {
        let mut type_state = ListState::default();
        type_state.select(Some(0));

        let mut category_state = ListState::default();
        category_state.select(Some(0));

        Self {
            running: false,
            active_field: 0,
            config_file,
            file_name_input: Input::default(),
            type_items: vec![
                "".to_string(),
                "Todos".to_string(),
                "Ideas".to_string(),
                "Notes".to_string(),
            ],
            type_state,
            category_items: vec![
                "".to_string(),
                "Project".to_string(),
                "Area".to_string(),
                "Resource".to_string(),
                "Archive".to_string(),
            ],
            category_state,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(10),
                Constraint::Length(3),
                Constraint::Percentage(70),
                Constraint::Percentage(10),
            ],
        )
        .split(frame.area());

        let form_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(50),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Percentage(10),
            ],
        )
        .split(main_layout[1]);

        let files_list_and_preview_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(10),
            ],
        )
        .split(main_layout[2]);

        let file_name_style = if self.active_field == 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        frame.render_widget(
            Paragraph::new(self.file_name_input.value()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("File name")
                    .style(file_name_style),
            ),
            form_layout[1],
        );

        // Type List
        let type_style = if self.active_field == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let type_items: Vec<ListItem> = self
            .type_items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let type_list = List::new(type_items)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Type")
                    .style(type_style),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::Green));
        frame.render_stateful_widget(type_list, form_layout[2], &mut self.type_state);

        // Category List
        let category_style = if self.active_field == 2 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let category_items: Vec<ListItem> = self
            .category_items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let category_list = List::new(category_items)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("PARA category")
                    .style(category_style),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::Green));
        frame.render_stateful_widget(category_list, form_layout[3], &mut self.category_state);

        frame.render_widget(
            Paragraph::new("List of files").block(Block::new().borders(Borders::ALL)),
            files_list_and_preview_layout[1],
        );
        frame.render_widget(
            Paragraph::new("File preview").block(Block::new().borders(Borders::ALL)),
            files_list_and_preview_layout[2],
        );

        let (cursor_x, cursor_y) = match self.active_field {
            0 => {
                let input = &self.file_name_input;
                (
                    form_layout[1].x + input.visual_cursor() as u16 + 1,
                    form_layout[1].y + 1,
                )
            }
            _ => (0, 0),
        };
        frame.set_cursor_position((cursor_x, cursor_y));
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Tab) => {
                self.active_field = (self.active_field + 1) % 3;
            }
            (_, KeyCode::Down | KeyCode::Char('j'))
                if self.active_field == 1 || self.active_field == 2 =>
            {
                match self.active_field {
                    1 => self.type_next(),
                    2 => self.category_next(),
                    _ => {}
                }
            }
            (_, KeyCode::Up | KeyCode::Char('k'))
                if self.active_field == 1 || self.active_field == 2 =>
            {
                match self.active_field {
                    1 => self.type_previous(),
                    2 => self.category_previous(),
                    _ => {}
                }
            }
            (_, KeyCode::Enter) if self.active_field == 0 => match self.selected_type() {
                Some("Todos") => {
                    let todo_file_name = self.generate_file_name();

                    let bufpath = PathBuf::from(format!(
                        "{}/{}",
                        self.config_file
                            .tino_dirs
                            .todos_dir
                            .into_path()
                            .unwrap()
                            .canonicalize()
                            .unwrap()
                            .display(),
                        todo_file_name
                    ));

                    File::create(bufpath).unwrap();
                }
                Some("Ideas") => {
                    let idea_file_name = self.generate_file_name();

                    let bufpath = PathBuf::from(format!(
                        "{}/{}",
                        self.config_file
                            .tino_dirs
                            .ideas_dir
                            .into_path()
                            .unwrap()
                            .canonicalize()
                            .unwrap()
                            .display(),
                        idea_file_name
                    ));

                    File::create(bufpath).unwrap();
                }
                Some("Notes") => {
                    let note_file_name = self.generate_file_name();

                    let bufpath = PathBuf::from(format!(
                        "{}/{}",
                        self.config_file
                            .tino_dirs
                            .ideas_dir
                            .into_path()
                            .unwrap()
                            .canonicalize()
                            .unwrap()
                            .display(),
                        note_file_name
                    ));

                    File::create(bufpath).unwrap();
                }
                // This should be like this because these aren't valid options for a file type,
                // code can be added to handle this cases but not creation of files (for now).
                Some("") => {}
                None => {}
                _ => {}
            },
            _ => {
                if self.active_field == 0 && key.code != KeyCode::Enter {
                    self.file_name_input.handle_event(&Event::Key(key));
                }
            }
        }
    }

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

    pub fn selected_type(&self) -> Option<&str> {
        self.type_state
            .selected()
            .map(|i| self.type_items[i].as_str())
    }

    pub fn selected_category(&self) -> Option<&str> {
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
