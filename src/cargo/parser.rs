use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub line: Option<String>,
    pub level: String,
    pub file: Option<String>,
    pub code: Option<String>,
    pub message: String,
}

impl Diagnostic {
    pub fn as_array(&self) -> [&str; 4] {
        [
            self.level.as_str(),
            self.code.as_deref().unwrap_or(""),
            self.file.as_deref().unwrap_or(""),
            self.line.as_deref().unwrap_or(""),
        ]
    }
}

pub fn parse(j: &str) -> Result<Option<Diagnostic>, AppError> {
    let json: serde_json::Value = serde_json::from_str(j)?;

    if json["reason"].as_str() != Some("compiler-message") {
        Ok(None)
    } else {
        let msg_block = &json["message"];

        let level = msg_block["level"]
            .as_str()
            .map(|v| v.to_string())
            .ok_or(AppError::ParseError("level missing".into()))?;

        if level == "failure-note" {
            return Ok(None);
        }

        let code_block = &msg_block["code"];

        let code = code_block["code"].as_str().map(|v| v.to_string());

        let span = &msg_block["spans"][0];

        let (file, line) = if span["is_primary"].as_bool() == Some(true) {
            let file = span["file_name"].as_str().map(|v| v.to_string());

            let line = span["line_start"].as_u64().map(|v| v.to_string());

            (file, line)
        } else {
            (None, None)
        };

        let message = msg_block["message"]
            .as_str()
            .map(|v| v.to_string())
            .ok_or(AppError::ParseError("message missing".into()))?;

        Ok(Some(Diagnostic {
            line,
            level,
            file,
            code,
            message,
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
        assert_eq!(d.message, "mismatched types");
    }

    #[test]
    fn parse_warning_test() {
        let j = include_str!("../cargo/test_json/test_warning.txt");

        let d = parse(j).unwrap().expect("none returned for valid input");
        assert_eq!(d.line.as_deref(), Some("6"));
        assert_eq!(d.level, "warning");
        assert_eq!(d.file.as_deref(), Some("src/main.rs"));
        assert_eq!(d.code.as_deref(), Some("unused_variables"));
        assert_eq!(d.message, "unused variable: `unused_variable`");
    }

    #[test]
    fn parse_note_test() {
        let j = include_str!("../cargo/test_json/test_note.txt");

        let d = parse(j).unwrap().expect("none returned for valid input");

        assert_eq!(d.line.as_deref(), Some("12"));
        assert_eq!(d.file.as_deref(), Some("src/main.rs"));
        assert_eq!(d.level, "note");
        assert_eq!(d.code, None);
        assert_eq!(d.message, "this error originates in the macro `vec`");
    }
}
