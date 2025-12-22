use std::env;
use std::process::Command;

use crate::app::config_file::ConfigFile;
use crate::app::utils::TinoError;
use crate::ratatui_app::{helper_methods::Helpers, types::App};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Alignment;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Wrap;
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(config_file: ConfigFile) -> anyhow::Result<Self> {
        let mut type_state = ListState::default();
        type_state.select(Some(0));

        let mut category_state = ListState::default();
        category_state.select(Some(0));

        let mut tino_files_state = ListState::default();
        tino_files_state.select(Some(0));

        Ok(Self {
            running: false,
            active_field: 0,
            open_editor: false,
            config_file: config_file.clone(),
            scroll_position: (0, 0),
            file_name_input: Input::default(),
            type_items: vec![
                "Todos".to_string(),
                "Ideas".to_string(),
                "Notes".to_string(),
                "Academic notes".to_string(),
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
            tino_files: Self::get_tino_files(config_file.clone())?,
            tino_files_state,
            file_to_preview: String::from("File preview"),
        })
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        if self.open_editor {
            let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

            Command::new(editor)
                .arg(match self.selected_tino_file() {
                    Some(tino_file) => String::from(tino_file),
                    None => return Err(TinoError::NotSelectedTinoFile.into()),
                })
                .status()
                .expect("ERROR: while openning editor.");
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
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
            ],
        )
        .split(main_layout[2]);

        let file_name_style = if self.active_field == 0 {
            Style::default().fg(Color::Magenta)
        } else {
            Style::default()
        };
        frame.render_widget(
            Paragraph::new(self.file_name_input.value().white()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("File name")
                    .style(file_name_style)
                    .title_bottom(Line::from("(Ctrl+n)").alignment(Alignment::Right)),
            ),
            form_layout[1],
        );

        // Type List
        let type_style = if self.active_field == 1 {
            Style::default().fg(Color::Magenta)
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
                    .style(type_style)
                    .title_bottom(Line::from("(Ctrl+t)").alignment(Alignment::Right)),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::Cyan));
        frame.render_stateful_widget(type_list, form_layout[2], &mut self.type_state);

        // Category List
        let category_style = if self.active_field == 2 {
            Style::default().fg(Color::Magenta)
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
                    .style(category_style)
                    .title_bottom(Line::from("(Ctrl+c)").alignment(Alignment::Right)),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::Cyan));
        frame.render_stateful_widget(category_list, form_layout[3], &mut self.category_state);

        // TINO files List
        let tino_files_style = if self.active_field == 3 {
            Style::default().fg(Color::Magenta)
        } else {
            Style::default()
        };
        let tino_files_items: Vec<ListItem> = self
            .tino_files
            .iter()
            .map(|i| ListItem::new(i.0.as_str()).white())
            .collect();
        let tino_files_list = List::new(tino_files_items)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("TINO files")
                    .style(tino_files_style)
                    .title_bottom(Line::from("(Ctrl+l)").alignment(Alignment::Right)),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::Cyan));
        frame.render_stateful_widget(
            tino_files_list,
            files_list_and_preview_layout[1],
            &mut self.tino_files_state,
        );

        let file_preview_style = if self.active_field == 4 {
            Style::default().fg(Color::Magenta)
        } else {
            Style::default()
        };
        frame.render_widget(
            Paragraph::new(Text::from(self.file_to_preview.clone()).bold().white())
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title_bottom(Line::from("(Ctrl+p)").alignment(Alignment::Right)),
                )
                .style(file_preview_style)
                .wrap(Wrap { trim: true })
                .scroll((self.scroll_position.0, 0)),
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
    fn handle_crossterm_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => Ok(()),
            Event::Resize(_, _) => Ok(()),
            _ => Ok(()),
        }
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc) => {
                self.quit();
                Ok(())
            }
            (_, KeyCode::Tab) => {
                self.active_field = (self.active_field + 1) % 5;
                Ok(())
            }
            (KeyModifiers::CONTROL, KeyCode::Char('n')) => {
                self.active_field = 0;
                Ok(())
            }
            (KeyModifiers::CONTROL, KeyCode::Char('t')) => {
                self.active_field = 1;
                Ok(())
            }
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                self.active_field = 2;
                Ok(())
            }
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                self.active_field = 3;
                Ok(())
            }
            (KeyModifiers::CONTROL, KeyCode::Char('p')) => {
                self.active_field = 4;
                Ok(())
            }
            (_, KeyCode::Down | KeyCode::Char('j'))
                if self.active_field == 1 || self.active_field == 2 || self.active_field == 3 =>
            {
                match self.active_field {
                    1 => {
                        self.type_next();
                        Ok(())
                    }
                    2 => {
                        self.category_next();
                        Ok(())
                    }
                    3 => {
                        self.tino_file_next();
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            (_, KeyCode::Up | KeyCode::Char('k'))
                if self.active_field == 1 || self.active_field == 2 || self.active_field == 3 =>
            {
                match self.active_field {
                    1 => {
                        self.type_previous();
                        Ok(())
                    }
                    2 => {
                        self.category_previous();
                        Ok(())
                    }
                    3 => {
                        self.tino_file_previous();
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            (_, KeyCode::Down | KeyCode::Char('j')) if self.active_field == 4 => {
                self.scroll_position.0 = self.scroll_position.0.saturating_add(1);
                Ok(())
            }
            (_, KeyCode::Up | KeyCode::Char('k')) if self.active_field == 4 => {
                self.scroll_position.0 = self.scroll_position.0.saturating_sub(1);
                Ok(())
            }
            (_, KeyCode::Enter) if self.active_field == 3 => {
                self.open_editor = true;
                self.running = false;
                Ok(())
            }
            (_, KeyCode::Enter) if self.active_field == 0 => match self.selected_type() {
                Some("Todos") => {
                    self.create_tino_file(self.config_file.tino_dirs.todos_dir.clone().as_str())
                }
                Some("Ideas") => {
                    self.create_tino_file(self.config_file.tino_dirs.ideas_dir.clone().as_str())
                }
                Some("Notes") => {
                    self.create_tino_file(self.config_file.tino_dirs.notes_dir.clone().as_str())
                }
                Some("Academic notes") => self.create_tino_file(
                    self.config_file
                        .tino_dirs
                        .academic_notes_dir
                        .clone()
                        .as_str(),
                ),
                //NOTE:
                // This should be like this because these aren't valid options for a file type,
                // code can be added to handle this cases but not creation of files (for now).
                None => Ok(()),
                _ => Ok(()),
            },
            (_, KeyCode::Char('v')) if self.active_field == 3 => {
                self.file_to_preview = self.get_file_content()?;
                self.scroll_position = (0, 0);
                Ok(())
            }
            _ => {
                if self.active_field == 0 && key.code != KeyCode::Enter {
                    self.file_name_input.handle_event(&Event::Key(key));
                }
                Ok(())
            }
        }
    }
}
