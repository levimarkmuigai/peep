use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

use crate::{app::AppState, ui::theme::Theme};

pub fn render_table(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    table_state: &mut TableState,
    theme: &Theme,
) {
    let header = ["level", "code", "file", "line"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);

    let rows = state.items.iter().map(|d| Row::new(d.as_array()).height(4));

    let widths = [
        Constraint::Length(9),
        Constraint::Length(7),
        Constraint::Min(12),
        Constraint::Length(6),
    ];

    let pointer = "▶ ";

    let highlight_symbol: Vec<Line> = vec![pointer.into(), "".into(), "".into(), "".into()];

    let t = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::new().fg(theme.border)),
        )
        .highlight_symbol(Text::from(highlight_symbol))
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

    frame.render_stateful_widget(t, area, table_state);
}
