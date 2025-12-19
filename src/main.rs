mod app;
mod ratatui_app;

use crate::{app::config_file::ConfigFile, ratatui_app::types::App};

fn main() -> anyhow::Result<()> {
    let config = ConfigFile::new(false)?;
    // NOTE: Run ratatui app
    color_eyre::install().unwrap();
    let terminal = ratatui::init();
    let result = App::new(config)?.run(terminal);
    ratatui::restore();
    result
}
