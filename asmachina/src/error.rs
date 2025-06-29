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
    #[error("Division by zero at address {address}")]
    DivisionByZero { address: u16 },
    #[error("Input/Output error: {message}")]
    IoError { message: String },
    #[error("Breakpoint hit at address {address}")]
    BreakpointHit { address: u16 },
    #[error("Invalid addressing mode: {mode}")]
    InvalidAddressingMode { mode: u8 },
    #[error("Invalid register number: {register} (must be 0-7)")]
    InvalidRegister { register: u8 },
}
