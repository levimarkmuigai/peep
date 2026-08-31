use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::AppState, ui::theme::Theme};

pub fn render_footer(frame: &mut Frame, area: Rect, _state: &AppState, theme: &Theme) {
    let text = vec![Line::from(vec![
        Span::styled("q QUIT", Style::new().fg(theme.detail_title).bold()),
        Span::raw(" | "),
        Span::styled("↑ MOVE-UP", Style::new().fg(theme.detail_title).bold()),
        Span::raw(" | "),
        Span::styled("↓ MOVE-DOWN", Style::new().fg(theme.detail_title).bold()),
        Span::raw(" | "),
        Span::styled(
            "j SCROLL-UP REC",
            Style::new().fg(theme.detail_title).bold(),
        ),
        Span::raw(" | "),
        Span::styled(
            "k SCROLL-DOWN REC",
            Style::new().fg(theme.detail_title).bold(),
        ),
    ])];

    let footer = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(theme.dim_border)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
