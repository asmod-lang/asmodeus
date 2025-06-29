use crate::error::MachineError;
use super::MachineW;

impl MachineW {
    pub(crate) fn push_to_stack(&mut self, value: u16) -> Result<(), MachineError> {
        if self.ws == 0 {
            return Err(MachineError::StackOverflow);
        }
        self.write_memory(self.ws, value)?;
        self.ws = self.ws.wrapping_sub(1);
        Ok(())
    }

    pub(crate) fn pop_from_stack(&mut self) -> Result<u16, MachineError> {
        if self.ws >= 2047 {
            return Err(MachineError::StackUnderflow);
        }
        self.ws = self.ws.wrapping_add(1);
        let value = self.read_memory(self.ws)?;
        Ok(value)
    }
}
