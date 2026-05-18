pub mod app;
pub mod cargo;
pub mod error;
pub mod events;
pub mod tui;
pub mod ui;

fn main() {
    let diag_vec = cargo::runner::run();
    println!("Here are the diagnostics: {:#?}", diag_vec);
}
