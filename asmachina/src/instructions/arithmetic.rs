use crate::error::MachineError;
use crate::machine::MachineW;
use shared::{extract_addressing_mode, addressing_mode_bits};

impl MachineW {
    /// DOD - Add: (AK) + ((AD)) → AK
    pub(crate) fn execute_dod(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let addressing_mode_bits = extract_addressing_mode(raw_instruction);
        
        let operand = if addressing_mode_bits == addressing_mode_bits::IMMEDIATE {
            self.ad
        } else {
            let effective_address = self.resolve_effective_address(raw_instruction)?;
            self.read_memory(effective_address)?
        };
        
        self.ak = self.ak.wrapping_add(operand);
        Ok(())
    }

    /// ODE - Subtract: (AK) - ((AD)) → AK  
    pub(crate) fn execute_ode(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let addressing_mode_bits = extract_addressing_mode(raw_instruction);
        
        let operand = if addressing_mode_bits == addressing_mode_bits::IMMEDIATE {
            self.ad
        } else {
            let effective_address = self.resolve_effective_address(raw_instruction)?;
            self.read_memory(effective_address)?
        };
        
        self.ak = self.ak.wrapping_sub(operand);
        Ok(())
    }
}
