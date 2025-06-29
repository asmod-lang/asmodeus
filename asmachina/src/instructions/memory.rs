use crate::error::MachineError;
use crate::machine::MachineW;
use shared::{extract_addressing_mode, addressing_mode_bits};

impl MachineW {
    /// ŁAD - Store: (AK) → (AD)
    pub(crate) fn execute_lad(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let effective_address = self.resolve_effective_address(raw_instruction)?;
        self.write_memory(effective_address, self.ak)
    }

    /// POB - Load: ((AD)) → AK
    pub(crate) fn execute_pob(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let addressing_mode_bits = extract_addressing_mode(raw_instruction);
        
        if addressing_mode_bits == addressing_mode_bits::IMMEDIATE {
            self.ak = self.ad;
        } else {
            let effective_address = self.resolve_effective_address(raw_instruction)?;
            self.ak = self.read_memory(effective_address)?;
        }
        
        Ok(())
    }
}
