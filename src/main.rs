use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
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
    // TODO: Make the TUI size smaller but according with the columns and lines of the terminal
    // so in smaller terminals will the TUI will be smaller and in bigger terminal will be bigger.
    // TODO: Read this https://ratatui.rs/faq#how-do-i-avoid-panics-due-to-out-of-range-calls-on-the-buffer
    // NOTE: https://crates.io/crates/tui-input for user text input.
    fn render(&mut self, frame: &mut Frame) {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
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

        frame.render_widget(
            Paragraph::new("File name").block(Block::new().borders(Borders::ALL)),
            form_layout[1],
        );
        frame.render_widget(
            Paragraph::new("Type").block(Block::new().borders(Borders::ALL)),
            form_layout[2],
        );
        frame.render_widget(
            Paragraph::new("PARA category").block(Block::new().borders(Borders::ALL)),
            form_layout[3],
        );

        frame.render_widget(
            Paragraph::new("List of files").block(Block::new().borders(Borders::ALL)),
            files_list_and_preview_layout[1],
        );
        frame.render_widget(
            Paragraph::new("File preview").block(Block::new().borders(Borders::ALL)),
            files_list_and_preview_layout[2],
        );
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
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
