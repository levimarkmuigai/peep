use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::{
    cargo::parser::{Diagnostics, parse},
    error::AppError,
};

pub fn run() -> Result<Vec<Diagnostics>, AppError> {
    let mut child = Command::new("cargo")
        .args(["check", "--message-format=json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let stdout = child.stdout.take().ok_or(AppError::StdoutUnavailable)?;
    let reader = BufReader::new(stdout);

    let diagnostics: Vec<Diagnostics> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            parse(&line).ok().flatten()
        })
        .collect();

    child.wait()?;

    Ok(diagnostics)
}
