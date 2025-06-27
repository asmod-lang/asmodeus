//! debugging utilities

use crate::error::MachineError;
use crate::machine::MachineW;
use crate::types::MachineWState;

impl MachineW {
    pub fn step_instruction(&mut self) -> Result<(), MachineError> {
        if !self.is_running {
            return Ok(());
        }

        if self.breakpoints.contains(&self.l) {
            return Err(MachineError::BreakpointHit { address: self.l });
        }

        self.step()
    }

    pub fn run_until_halt_or_breakpoint(&mut self) -> Result<(), MachineError> {
        while self.is_running {

            if self.breakpoints.contains(&self.l) {
                return Err(MachineError::BreakpointHit { address: self.l });
            }

            self.step()?;
        }
        Ok(())
    }

    /// snapshot
    pub fn get_current_state(&self) -> MachineWState {
        MachineWState {
            ak: self.ak,
            l: self.l,
            ad: self.ad,
            kod: self.kod,
            ws: self.ws,
            is_running: self.is_running,
            interrupts_enabled: self.interrupts_enabled,
            interrupt_mask: self.interrupt_mask,
            registers: self.registers,
        }
    }

    pub fn get_memory_range(&self, start_addr: u16, end_addr: u16) -> Option<Vec<(u16, u16)>> {
        if start_addr > end_addr || end_addr >= 2048 {
            return None;
        }

        let mut result = Vec::new();
        for addr in start_addr..=end_addr {
            result.push((addr, self.memory[addr as usize]));
        }
        Some(result)
    }

    pub fn add_breakpoint(&mut self, address: u16) -> Result<(), MachineError> {
        if address >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address });
        }
        self.breakpoints.insert(address);
        Ok(())
    }

    pub fn has_breakpoint(&self, address: u16) -> bool {
        self.breakpoints.contains(&address)
    }

    pub fn remove_breakpoint(&mut self, address: u16) -> bool {
        self.breakpoints.remove(&address)
    }

    pub fn list_breakpoints(&self) -> Vec<u16> {
        let mut breakpoints: Vec<u16> = self.breakpoints.iter().copied().collect();
        breakpoints.sort();
        breakpoints
    }

    pub fn clear_all_breakpoints(&mut self) {
        self.breakpoints.clear();
    }
}
