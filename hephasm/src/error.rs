//! error types for hephasm

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AssemblerError {
    #[error("Undefined symbol: {symbol} at line {line}")]
    UndefinedSymbol { symbol: String, line: usize },
    #[error("Duplicate symbol definition: {symbol} at line {line}")]
    DuplicateSymbol { symbol: String, line: usize },
    #[error("Invalid opcode: {opcode} at line {line}")]
    InvalidOpcode { opcode: String, line: usize },
    #[error("Invalid number format: {value} at line {line}")]
    InvalidNumber { value: String, line: usize },
    #[error("Address out of bounds: {address} (max 2047) at line {line}")]
    AddressOutOfBounds { address: u16, line: usize },
    #[error("Invalid addressing mode for instruction {instruction}: {mode} at line {line}")]
    InvalidAddressingMode { instruction: String, mode: String, line: usize },
    #[error("Macro not found: {name} at line {line}")]
    MacroNotFound { name: String, line: usize },
    #[error("Macro parameter count mismatch for {name}: expected {expected}, found {found} at line {line}")]
    MacroParameterMismatch { name: String, expected: usize, found: usize, line: usize },
    #[error("Memory overflow: program too large for available memory")]
    MemoryOverflow,
    #[error("Extended instruction '{instruction}' not enabled at line {line}. Use --extended flag to enable extended instruction set")]
    ExtendedInstructionNotEnabled { instruction: String, line: usize },
}
