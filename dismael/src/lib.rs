//! Dismael - Disassembler for Machine W binary code
//! Converts binary machine code back to readable Asmodeus assembly

mod error;
mod types;
mod instruction;
mod analyzer;
mod formatter;
mod core;
mod advanced_disassembler;
mod ascii_art;

pub use error::DisassemblerError;
pub use types::{DisassembledInstruction, AddressingMode};
pub use core::Disassembler;
pub use advanced_disassembler::AdvancedDisassembler;

pub fn disassemble(machine_code: &[u16]) -> Result<Vec<String>, DisassemblerError> {
    let mut disassembler = Disassembler::new();
    disassembler.disassemble(machine_code)
}

pub fn disassemble_to_string(machine_code: &[u16]) -> Result<String, DisassemblerError> {
    let lines = disassemble(machine_code)?;
    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dismael_logo() {
        crate::ascii_art::print_dismael_logo();
        assert!(true);
    }
}
