//! Error types for the parser

use lexariel::LexerError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Unexpected token at line {line}, column {column}: expected {expected}, found {found}")]
    UnexpectedToken {
        line: usize,
        column: usize,
        expected: String,
        found: String,
    },
    #[error("Unexpected end of file, expected {expected}")]
    UnexpectedEof { expected: String },
    #[error("Invalid addressing mode at line {line}, column {column}: {mode}")]
    InvalidAddressingMode {
        line: usize,
        column: usize,
        mode: String,
    },
    #[error("Missing operand for instruction {instruction} at line {line}, column {column}")]
    MissingOperand {
        instruction: String,
        line: usize,
        column: usize,
    },
    #[error("Invalid macro definition at line {line}, column {column}: {message}")]
    InvalidMacroDefinition {
        line: usize,
        column: usize,
        message: String,
    },
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),
}
