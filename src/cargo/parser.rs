use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct Diagnostics {
    pub line: Option<String>,
    pub level: String,
    pub file: Option<String>,
    pub code: Option<String>,
    pub explanation: Option<String>,
}

pub fn parse(j: &str) -> Result<Option<Diagnostics>, AppError> {
    let json: serde_json::Value = serde_json::from_str(j)?;

    if json["reason"].as_str() != Some("compiler-message") {
        Ok(None)
    } else {
        let message = &json["message"];

        let level = message["level"]
            .as_str()
            .map(|v| v.to_string())
            .ok_or(AppError::ParseError("level missing".into()))?;

        let code_block = &message["code"];

        let code = code_block["code"].as_str().map(|v| v.to_string());

        let explanation = code_block["explanation"].as_str().map(|v| v.to_string());

        let span = &message["spans"][0];

        let (file, line) = if span["is_primary"] == true {
            let file = span["file_name"].as_str().map(|v| v.to_string());

            let line = span["line_start"].as_u64().map(|v| v.to_string());

            (file, line)
        } else {
            (None, None)
        };

        Ok(Some(Diagnostics {
            line,
            level,
            file,
            code,
            explanation,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_test() {
        let j = include_str!("../cargo/test_json/test_error.txt");

        let d = parse(j).unwrap().expect("none returned for valid input");
        assert_eq!(d.line.as_deref(), Some("10"));
        assert_eq!(d.level, "error");
        assert_eq!(d.code.as_deref(), Some("E0308"));
        assert_eq!(d.file.as_deref(), Some("src/main.rs"));
        assert_eq!(
            d.explanation.as_deref(),
            Some("Expected type did not match the received type...\n\n(Truncated for readability)")
        );
    }

    #[test]
    fn parse_warning_test() {
        let j = include_str!("../cargo/test_json/test_warning.txt");

        let d = parse(j).unwrap().expect("none returned for valid input");
        assert_eq!(d.line.as_deref(), Some("6"));
        assert_eq!(d.level, "warning");
        assert_eq!(d.file.as_deref(), Some("src/main.rs"));
        assert_eq!(d.code.as_deref(), Some("unused_variables"));
        assert_eq!(d.explanation, None);
    }

    #[test]
    fn parse_note_test() {
        let j = include_str!("../cargo/test_json/test_note.txt");

        let d = parse(j).unwrap().expect("none returned for valid input");

        assert_eq!(d.line.as_deref(), Some("12"));
        assert_eq!(d.file.as_deref(), Some("src/main.rs"));
        assert_eq!(d.level, "note");
        assert_eq!(d.code, None);
        assert_eq!(d.explanation, None);
    }
}
