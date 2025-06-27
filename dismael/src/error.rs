//! error types for dismael disassembler

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DisassemblerError {
    #[error("Invalid opcode: {opcode} at address {address}")]
    InvalidOpcode { opcode: u8, address: u16 },
    #[error("Empty machine code")]
    EmptyCode,
    #[error("Address out of bounds: {address}")]
    AddressOutOfBounds { address: u16 },
}
