//! Error handling for Asmodeus CLI

use std::fmt;

#[derive(Debug)]
pub enum AsmodeusError {
    IoError(std::io::Error),
    LexerError(lexariel::LexerError),
    ParserError(parseid::ParserError),
    AssemblerError(hephasm::AssemblerError),
    MachineError(asmachina::MachineError),
    DisassemblerError(dismael::DisassemblerError),
    UsageError(String),
}

impl fmt::Display for AsmodeusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmodeusError::IoError(e) => write!(f, "I/O Error: {}", e),
            AsmodeusError::LexerError(e) => write!(f, "Lexer Error: {}", e),
            AsmodeusError::ParserError(e) => write!(f, "Parser Error: {}", e),
            AsmodeusError::AssemblerError(e) => write!(f, "Assembler Error: {}", e),
            AsmodeusError::MachineError(e) => write!(f, "Machine Error: {}", e),
            AsmodeusError::DisassemblerError(e) => write!(f, "Disassembler Error: {}", e),
            AsmodeusError::UsageError(e) => write!(f, "Usage Error: {}", e),
        }
    }
}

impl std::error::Error for AsmodeusError {}

impl From<std::io::Error> for AsmodeusError {
    fn from(error: std::io::Error) -> Self {
        AsmodeusError::IoError(error)
    }
}

impl From<lexariel::LexerError> for AsmodeusError {
    fn from(error: lexariel::LexerError) -> Self {
        AsmodeusError::LexerError(error)
    }
}

impl From<parseid::ParserError> for AsmodeusError {
    fn from(error: parseid::ParserError) -> Self {
        AsmodeusError::ParserError(error)
    }
}

impl From<hephasm::AssemblerError> for AsmodeusError {
    fn from(error: hephasm::AssemblerError) -> Self {
        AsmodeusError::AssemblerError(error)
    }
}

impl From<asmachina::MachineError> for AsmodeusError {
    fn from(error: asmachina::MachineError) -> Self {
        AsmodeusError::MachineError(error)
    }
}

impl From<dismael::DisassemblerError> for AsmodeusError {
    fn from(error: dismael::DisassemblerError) -> Self {
        AsmodeusError::DisassemblerError(error)
    }
}
