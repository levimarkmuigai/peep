use crossterm::event::{Event, KeyCode, KeyEventKind};

#[derive(Debug)]
pub enum AppAction {
    Quit,
    SelectUp,
    SelectDown,
    None,
}

pub fn map_event(event: Event) -> AppAction {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => AppAction::Quit,
            KeyCode::Up | KeyCode::Char('k') => AppAction::SelectUp,
            KeyCode::Down | KeyCode::Char('j') => AppAction::SelectDown,
            _ => AppAction::None,
        },
        _ => AppAction::None,
    }
}
