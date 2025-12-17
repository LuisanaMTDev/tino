mod app;
mod ratatui_app;

use crate::{app::config_file::ConfigFile, ratatui_app::app_and_rust_traits_impls::App};

fn main() -> color_eyre::Result<()> {
    let config = ConfigFile::new(false).unwrap();
    // NOTE: Run ratatui app
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(config).run(terminal);
    ratatui::restore();
    result
}
