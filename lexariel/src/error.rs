//! error types for lexer

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LexerError {
    #[error("Unknown token at line {line}, column {column}: '{token}'")]
    UnknownToken { line: usize, column: usize, token: String },
    #[error("Invalid number format at line {line}, column {column}: '{value}'")]
    InvalidNumberFormat { line: usize, column: usize, value: String },
    #[error("Unterminated string at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },
    #[error("Invalid character at line {line}, column {column}: '{character}'")]
    InvalidCharacter { line: usize, column: usize, character: char },
}
