pub mod app;
pub mod cargo;
pub mod error;
pub mod events;
pub mod tui;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    let diagnostics = cargo::runner::run()?;

    let state = app::AppState::new(diagnostics);

    let mut terminal = tui::init()?;

    let theme = ui::theme::Theme::default();

    app::run(state, &mut terminal, &theme)?;

    ratatui::restore();

    Ok(())
}
