use crate::error::MachineError;
use super::MachineW;

impl MachineW {
    pub fn trigger_interrupt(&mut self, interrupt_vector_address: u16) {
        if self.interrupts_enabled {
            self.pending_interrupt = Some(interrupt_vector_address);
        }
    }

    fn _handle_interrupts(&mut self) -> Result<(), MachineError> {
        if self.interrupts_enabled && self.pending_interrupt.is_some() {
            let interrupt_vector = self.pending_interrupt.take().unwrap();
            
            self.push_to_stack(self.ak)?;
            self.push_to_stack(self.l)?;
            
            self.interrupts_enabled = false;
            self.l = interrupt_vector & 0b0000011111111111;
        }
        Ok(())
    }
}
