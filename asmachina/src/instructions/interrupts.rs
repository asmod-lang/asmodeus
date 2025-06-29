use crate::error::MachineError;
use crate::machine::MachineW;

impl MachineW {
    /// DNS - Disable interrupts
    pub(crate) fn execute_dns(&mut self) -> Result<(), MachineError> {
        self.interrupts_enabled = false;
        Ok(())
    }

    /// CZM - Clear interrupt mask
    pub(crate) fn execute_czm(&mut self) -> Result<(), MachineError> {
        self.interrupt_mask = 0;
        Ok(())
    }

    /// MSK - Set interrupt mask
    pub(crate) fn execute_msk(&mut self) -> Result<(), MachineError> {
        self.interrupt_mask = self.ad;
        Ok(())
    }

    /// PWR - Return from interrupt
    pub(crate) fn execute_pwr(&mut self) -> Result<(), MachineError> {
        // restore state from stack 
        self.l = self.pop_from_stack()?;
        self.ak = self.pop_from_stack()?;
        self.l = self.l & 0b0000011111111111;
        self.interrupts_enabled = true;
        Ok(())
    }
}
