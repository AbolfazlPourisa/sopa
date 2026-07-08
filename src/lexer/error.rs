use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Invalid type '{error}' at line {line_number}:\n  {line}\n expected a valid type")]
    InvalidType {
        line_number: usize,
        line: String,
        error: String,
    },

    #[error("Invalid string at line {line_number}:\n  {line}\n missing closing quote or invalid escape sequence")]
    InvalidString {
        line_number: usize,
        line: String,
        error: String,
    }
}