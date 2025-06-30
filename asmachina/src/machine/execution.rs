use crate::error::MachineError;
use asmodeus_shared::{extract_opcode, extract_argument};
use super::MachineW;

impl MachineW {
    fn fetch_and_decode(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.read_memory(self.l)?;
        
        self.kod = extract_opcode(raw_instruction);
        self.ad = extract_argument(raw_instruction);
        
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), MachineError> {
        if !self.is_running {
            return Ok(());
        }

        // checking for pending interrupts before executing instrunction 
        if self.interrupts_enabled && self.pending_interrupt.is_some() {
            let interrupt_vector = self.pending_interrupt.take().unwrap();
            
            // saving current state on stack 
            self.push_to_stack(self.ak)?;
            self.push_to_stack(self.l)?; // current L, not the incremented one
            
            // disable interrupts and jump to interrupt handler
            self.interrupts_enabled = false;
            self.l = interrupt_vector & 0b0000011111111111;
            return Ok(()); // dont execute normal instruction this cycle
        }

        self.fetch_and_decode()?;
        
        // increment instruction counter (before execution, may be overridden by jumps)
        self.l = (self.l + 1) & 0b0000011111111111;
        
        self.execute_instruction() // based on the decoded opcode
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        self.is_running = true;
        
        while self.is_running {
            self.step()?;
        }
        
        Ok(())
    }

    pub fn run_steps(&mut self, max_steps: usize) -> Result<usize, MachineError> {
        self.is_running = true;
        let mut steps = 0;
        
        while self.is_running && steps < max_steps {
            self.step()?;
            steps += 1;
        }
        
        Ok(steps)
    }
}
