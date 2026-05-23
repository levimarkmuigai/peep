pub mod app;
pub mod cargo;
pub mod error;
pub mod events;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    let diagnostics = cargo::runner::run()?;

    let state = app::AppState::new(diagnostics);

    let mut terminal = ui::tui::init()?;

    app::run(state, &mut terminal)?;

    ratatui::restore();

    Ok(())
}
