use ratatui::style::{Color, palette::tailwind};

pub struct Theme {
    pub border: Color,
    pub dim_border: Color,
    pub app_title: Color,
    pub stats_warning: Color,
    pub stats_error: Color,
    pub detail_title: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            border: tailwind::GRAY.c600,
            dim_border: tailwind::NEUTRAL.c200,
            app_title: tailwind::SLATE.c500,
            stats_warning: tailwind::AMBER.c200,
            stats_error: tailwind::RED.c200,
            detail_title: tailwind::GRAY.c400,
        }
    }
}
