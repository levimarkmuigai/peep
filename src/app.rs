use crossterm::event;
use ratatui::widgets::TableState;

use crate::{
    Recommendation,
    events::handler::{self, AppAction},
    tui::Tui,
    ui::{self, theme::Theme},
};

pub struct AppState {
    pub items: Vec<Recommendation>,
    pub table: TableState,
    pub should_quit: bool,
    pub rec_scroll: u16,
}

impl AppState {
    pub fn new(items: Vec<Recommendation>) -> Self {
        let mut table = TableState::default();

        if !items.is_empty() {
            table.select(Some(0));
        }
        Self {
            items,
            table,
            should_quit: false,
            rec_scroll: 0,
        }
    }

    pub fn update(&mut self, action: AppAction) {
        match action {
            AppAction::SelectUp => {
                self.table.select_previous();
                self.rec_scroll = 0;
            }
            AppAction::SelectDown => {
                self.table.select_next();
                self.rec_scroll = 0;
            }
            AppAction::Quit => {
                self.should_quit = true;
            }

            AppAction::ScrollUp => {
                self.rec_scroll = self.rec_scroll.saturating_sub(2);
            }
            AppAction::ScrollDown => {
                self.rec_scroll = self.rec_scroll.saturating_sub(2);
            }
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
