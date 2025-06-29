use crate::error::MachineError;
use crate::machine::MachineW;

impl MachineW {
    /// SOB - Unconditional jump: (AD) → L
    pub(crate) fn execute_sob(&mut self) -> Result<(), MachineError> {
        self.l = self.ad & 0b0000011111111111;
        Ok(())
    }

    /// SOM - Conditional jump: (AD) → L, when (AK) < 0
    pub(crate) fn execute_som(&mut self) -> Result<(), MachineError> {
        // checking if the AK value is negative 
        if (self.ak & 0x8000) != 0 {
            self.l = self.ad & 0b0000011111111111;
        }
        Ok(())
    }

    /// SOZ - Conditional jump: (AD) → L, when (AK) = 0
    pub(crate) fn execute_soz(&mut self) -> Result<(), MachineError> {
        // AK == zero
        if self.ak == 0 {
            self.l = self.ad & 0b0000011111111111;
        }
        Ok(())
    }

    /// STP - Stop
    pub(crate) fn execute_stp(&mut self) -> Result<(), MachineError> {
        self.is_running = false;
        Ok(())
    }
}
