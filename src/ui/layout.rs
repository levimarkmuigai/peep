use ratatui::layout::{Constraint, Layout, Rect};

pub fn areas(area: Rect) -> (Rect, Rect, Rect, Rect) {
    let [statbar, table, details, footer] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .areas(area);

    (statbar, table, details, footer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_tiles_correctly() {
        let terminal = Rect::new(0, 0, 120, 40);
        let (statbar, table, details, footer) = areas(terminal);

        assert_eq!(statbar.y, 0);
        assert_eq!(statbar.height, 2);

        assert_eq!(table.y, 2);
        assert_eq!(table.height, 31);

        assert_eq!(details.y, 33);
        assert_eq!(details.height, 6);

        assert_eq!(footer.y, 39);
        assert_eq!(footer.height, 1);

        assert_eq!(statbar.width, 120);
        assert_eq!(table.width, 120);
        assert_eq!(details.width, 120);
        assert_eq!(footer.width, 120);

        assert_eq!(
            statbar.height + table.height + details.height + footer.height,
            terminal.height
        );
    }
}
