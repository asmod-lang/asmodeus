//! Core emulator for Machine W architecture
//! 
//! - 16-bit word size architecture
//! - 2048 words of memory
//! - Full instruction set with arithmetic, logic, control flow, and I/O operations
//! - Interactive and batch execution modes
//! - Debugging support with breakpoints
//! - Interrupt handling system

mod error;
mod types;
mod machine;
mod instructions;
mod debug;

pub use error::MachineError;
pub use types::{AddressingMode, MachineWState};
pub use machine::MachineW;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_creation() {
        let machine = MachineW::new();
        assert_eq!(machine.ak, 0);
        assert_eq!(machine.l, 0);
        assert_eq!(machine.is_running, false);
    }

    #[test]
    fn test_addressing_mode_immediate() {
        let addr_mode = AddressingMode::Immediate(42);
        match addr_mode {
            AddressingMode::Immediate(value) => assert_eq!(value, 42),
            _ => panic!("Expected immediate addressing mode"),
        }
    }

    #[test]
    fn test_error_types() {
        let error = MachineError::AddressOutOfBounds { address: 3000 };
        assert!(error.to_string().contains("3000"));
        assert!(error.to_string().contains("out of bounds"));
    }
}
