use crossterm::event;
use ratatui::widgets::TableState;

use crate::{
    cargo::parser::Diagnostic,
    events::handler::{self, AppAction},
    tui::Tui,
    ui::{self, theme::Theme},
};

pub struct AppState {
    pub items: Vec<Diagnostic>,
    pub table: TableState,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(items: Vec<Diagnostic>) -> Self {
        let mut table = TableState::default();

        if !items.is_empty() {
            table.select(Some(0));
        }
        Self {
            items,
            table,
            should_quit: false,
        }
    }

    pub fn update(&mut self, action: AppAction) {
        match action {
            AppAction::SelectUp => self.table.select_previous(),
            AppAction::SelectDown => self.table.select_next(),
            AppAction::Quit => self.should_quit = true,
            AppAction::None => {}
        }
    }
}

pub fn run(mut state: AppState, terminal: &mut Tui, theme: &Theme) -> color_eyre::Result<()> {
    while !state.should_quit {
        terminal.draw(|frame| {
            ui::render(frame, &mut state, theme);
        })?;

        let event = event::read()?;
        let action = handler::map_event(event);
        state.update(action);
    }

    Ok(())
}
