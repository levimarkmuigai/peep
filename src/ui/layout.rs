use ratatui::layout::{Constraint, Layout, Rect};

pub fn areas(area: Rect) -> (Rect, Rect, Rect) {
    let [statbar, table, details] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(6),
    ])
    .areas(area);

    (statbar, table, details)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_tiles_correctly() {
        let terminal = Rect::new(0, 0, 120, 40);
        let (statbar, table, details) = areas(terminal);

        assert_eq!(statbar.y, 0);
        assert_eq!(statbar.height, 1);

        assert_eq!(table.y, 1);
        assert_eq!(table.height, 33);

        assert_eq!(details.y, 34);
        assert_eq!(details.height, 6);

        assert_eq!(statbar.width, 120);
        assert_eq!(table.width, 120);
        assert_eq!(details.width, 120);

        assert_eq!(
            statbar.height + table.height + details.height,
            terminal.height
        );
    }
}
