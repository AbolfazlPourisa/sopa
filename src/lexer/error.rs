use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Invalid type {value}")]
    InvalidType { value: String },

    #[error("Invalid string")]
    InvalidString(),

    #[error("Invalid number")]
    InvalidNumber(),

    #[error("Invalid int")]
    InvalidInt(),

    #[error("Invalid float")]
    InvalidFloat()
}