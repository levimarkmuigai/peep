use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::AppState, ui::theme::Theme};

pub fn render_statbar(frame: &mut Frame, area: Rect, state: &AppState, theme: &Theme) {
    let [left, right] =
        Layout::horizontal([Constraint::Min(0), Constraint::Length(20)]).areas(area);

    let title = Paragraph::new(Line::from(Span::styled(
        "PEEP",
        Style::new().fg(theme.app_title).bold(),
    )));

    frame.render_widget(title, left);

    let error_count = state.items.iter().filter(|d| d.level == "error").count();
    let warning_count = state.items.iter().filter(|d| d.level == "warning").count();

    let error_text = if error_count == 1 {
        format!("{} err", error_count)
    } else {
        format!("{} errs", error_count)
    };

    let warning_text = if warning_count == 1 {
        format!("{} warn", warning_count)
    } else {
        format!("{} warns", warning_count)
    };

    let text = vec![Line::from(vec![
        Span::styled(error_text, Style::new().fg(theme.stats_error).bold()),
        Span::raw("   "),
        Span::styled(warning_text, Style::new().fg(theme.stats_warning).bold()),
    ])];

    let stats = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(theme.border),
        )
        .alignment(Alignment::Right);

    frame.render_widget(stats, right);
}
