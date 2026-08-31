pub mod api;
pub mod app;
pub mod cargo;
pub mod error;
pub mod events;
pub mod tui;
pub mod ui;

#[derive(Clone, Debug)]
pub struct Recommendation {
    pub line: Option<String>,
    pub level: String,
    pub file: Option<String>,
    pub code: Option<String>,
    pub rec: String,
}

impl Recommendation {
    pub fn as_array(&self) -> [&str; 4] {
        [
            self.level.as_str(),
            self.line.as_deref().unwrap_or(""),
            self.file.as_deref().unwrap_or(""),
            self.code.as_deref().unwrap_or(""),
        ]
    }
}

fn main() -> color_eyre::Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let diagnostics = cargo::runner::run()?;

    let mut recommendations: Vec<Recommendation> = Vec::with_capacity(diagnostics.len());

    if !diagnostics.is_empty() {
        for d in diagnostics {
            let rec = api::send_request(format!("{}: {}", d.level, d.message))?;

            recommendations.push(Recommendation {
                line: d.line,
                level: d.level,
                file: d.file,
                code: d.code,
                rec,
            });
        }
    }

    let state = app::AppState::new(recommendations);

    let mut terminal = tui::init()?;

    let theme = ui::theme::Theme::default();

    app::run(state, &mut terminal, &theme)?;

    ratatui::restore();

    Ok(())
}
