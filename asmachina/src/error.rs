//! error types emulator

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MachineError {
    #[error("Memory address out of bounds: {address}, valid range: 0-2047")]
    AddressOutOfBounds { address: u16 },
    #[error("Invalid opcode: {opcode}")]
    InvalidOpcode { opcode: u8 },
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid addressing mode for instruction")]
    InvalidAddressingMode,
    #[error("Input/Output error: {message}")]
    IoError { message: String },
    #[error("Breakpoint hit at address {address}")]
    BreakpointHit { address: u16 },
}
