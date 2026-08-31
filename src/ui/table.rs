use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::{app::AppState, ui::theme::Theme};

pub fn render_table(frame: &mut Frame, area: Rect, state: &mut AppState, theme: &Theme) {
    let header = ["level", "line", "file", "code"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);

    let rows = state.items.iter().map(|r| Row::new(r.as_array()).height(2));

    let widths = [
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Length(17),
        Constraint::Fill(1),
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

    frame.render_stateful_widget(t, area, &mut state.table);
}
