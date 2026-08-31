use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Paragraph, Wrap},
};

use crate::{app::AppState, ui::theme::Theme};

pub fn render_recommendations(frame: &mut Frame, area: Rect, state: &AppState, theme: &Theme) {
    let selected_diag = state.table.selected().and_then(|i| state.items.get(i));

    match selected_diag {
        Some(r) => {
            let file = r.file.as_deref().unwrap_or("");
            let line = r.line.as_deref().unwrap_or("");

            let details = Paragraph::new(r.rec.as_str())
                .block(
                    Block::default()
                        .title(format!(" {} : {} ", file, line))
                        .title_style(theme.detail_title),
                )
                .wrap(Wrap { trim: false })
                .scroll((state.rec_scroll, 0));

            frame.render_widget(details, area);
        }
        None => {
            let details = Paragraph::new("No diagnostic selected")
                .style(Style::new().add_modifier(Modifier::DIM));

            frame.render_widget(details, area);
        }
    }
}
