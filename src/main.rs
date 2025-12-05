mod ratatui_app;

use crate::ratatui_app::app_and_rust_traits_impls::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
