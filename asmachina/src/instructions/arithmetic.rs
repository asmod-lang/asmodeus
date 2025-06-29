use crate::error::MachineError;
use crate::machine::MachineW;

impl MachineW {
    /// DOD - Add: (AK) + ((AD)) → AK
    pub(crate) fn execute_dod(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let effective_address = self.resolve_effective_address(raw_instruction)?;
        let operand = self.read_memory(effective_address)?;
        self.ak = self.ak.wrapping_add(operand);
        Ok(())
    }

    /// ODE - Subtract: (AK) - ((AD)) → AK  
    pub(crate) fn execute_ode(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let effective_address = self.resolve_effective_address(raw_instruction)?;
        let operand = self.read_memory(effective_address)?;
        self.ak = self.ak.wrapping_sub(operand);
        Ok(())
    }
}
