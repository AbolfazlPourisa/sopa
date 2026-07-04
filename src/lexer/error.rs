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
    },

    #[error("Invalid number '{error}' at line {line_number}:\n {line}\n expected a valid number")]
    InvalidNumber {
        line_number: usize,
        line: String,
        error: String,
    },

    #[error("Invalid integer '{error}' at line {line_number}:\n  {line}\n expected an integer (digits only)")]
    InvalidInt {
        line_number: usize,
        line: String,
        error: String,
    },

    #[error("Invalid float '{error}' at line {line_number}:\n  {line}\n expected a float (e.g., '123.45')")]
    InvalidFloat {
        line_number: usize,
        line: String,
        error: String,
    },
}