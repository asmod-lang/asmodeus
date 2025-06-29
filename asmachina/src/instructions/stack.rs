use crate::error::MachineError;
use crate::machine::MachineW;

impl MachineW {
    /// PZS - Pop from stack: WS++, memory[WS] → AK
    pub(crate) fn execute_pzs(&mut self) -> Result<(), MachineError> {
        self.ak = self.pop_from_stack()?;
        Ok(())
    }

    /// SDP - Push to stack: (AK) → memory[WS], WS--
    pub(crate) fn execute_sdp(&mut self) -> Result<(), MachineError> {
        self.push_to_stack(self.ak)?;
        Ok(())
    }
}
