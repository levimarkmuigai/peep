use crossterm::event::{Event, KeyCode, KeyEventKind};

#[derive(Debug, PartialEq)]
pub enum AppAction {
    Quit,
    SelectUp,
    SelectDown,
    ScrollUp,
    ScrollDown,
    None,
}

pub fn map_event(event: Event) -> AppAction {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => AppAction::Quit,
            KeyCode::Up => AppAction::SelectUp,
            KeyCode::Down => AppAction::SelectDown,

            KeyCode::Char('j') => AppAction::ScrollUp,
            KeyCode::Char('k') => AppAction::ScrollDown,
            _ => AppAction::None,
        },
        _ => AppAction::None,
    }
}

#[cfg(test)]
mod test {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    use super::*;

    #[test]
    fn test_quit_action() {
        let q_press = Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        let esc_press = Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        assert_eq!(map_event(q_press), AppAction::Quit);
        assert_eq!(map_event(esc_press), AppAction::Quit);
    }

    #[test]
    fn test_select_up() {
        let up_press = Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        let k_press = Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        assert_eq!(map_event(up_press), AppAction::SelectUp);
        assert_eq!(map_event(k_press), AppAction::SelectUp);
    }

    #[test]
    fn test_select_down() {
        let down_press = Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        let j_press = Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        assert_eq!(map_event(down_press), AppAction::SelectDown);
        assert_eq!(map_event(j_press), AppAction::SelectDown);
    }
}
