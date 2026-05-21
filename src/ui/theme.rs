use ratatui::style::{Color, palette::tailwind};

pub struct Theme {
    pub border: Color,
    pub app_title: Color,
    pub stats_warning: Color,
    pub stats_error: Color,
    pub detail_title: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            border: tailwind::GRAY.c600,
            app_title: tailwind::FUCHSIA.c300,
            stats_warning: tailwind::AMBER.c500,
            stats_error: tailwind::RED.c500,
            detail_title: tailwind::GRAY.c400,
        }
    }
}
