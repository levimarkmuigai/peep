use ratatui::Frame;

use crate::{app::AppState, ui::theme::Theme};

pub mod detail;
pub mod footer;
pub mod layout;
pub mod statbar;
pub mod table;
pub mod theme;

pub fn render(frame: &mut Frame, state: &mut AppState, theme: &Theme) {
    let (stat_area, table_area, detail_area, footer_area) = layout::areas(frame.area());

    statbar::render_statbar(frame, stat_area, state, theme);
    table::render_table(frame, table_area, state, theme);
    detail::render_details(frame, detail_area, state, theme);
    footer::render_footer(frame, footer_area, state, theme);
}
