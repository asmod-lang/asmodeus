mod arithmetic;
mod memory;
mod control;
mod stack;
mod interrupts;
mod io;
mod extended_arithmetics;

use crate::error::MachineError;
use crate::machine::MachineW;

impl MachineW {
    pub(crate) fn execute_instruction(&mut self) -> Result<(), MachineError> {
        match self.kod {
            // Arithmetic instructions
            0b00001 => self.execute_dod(), // DOD - Add
            0b00010 => self.execute_ode(), // ODE - Subtract  
            
            // Memory instructions
            0b00011 => self.execute_lad(), // ŁAD - Store
            0b00100 => self.execute_pob(), // POB - Load
            
            // Control flow instructions
            0b00101 => self.execute_sob(), // SOB - Unconditional jump
            0b00110 => self.execute_som(), // SOM - Conditional jump
            0b10000 => self.execute_soz(), // SOZ - Jump if zero
            0b00111 => self.execute_stp(), // STP - Stop
            
            // Stack instructions
            0b01001 => self.execute_pzs(), // PZS - Pop from stack
            0b01010 => self.execute_sdp(), // SDP - Push to stack
            
            // Interrupt instructions
            0b01000 => self.execute_dns(), // DNS - Disable interrupts
            0b01011 => self.execute_czm(), // CZM - Clear interrupt mask
            0b01100 => self.execute_msk(), // MSK - Set interrupt mask
            0b01101 => self.execute_pwr(), // PWR - Return from interrupt
            
            // I/O instructions
            0b01110 => self.execute_wejscie(), // WEJSCIE - Input
            0b01111 => self.execute_wyjscie(), // WYJSCIE - Output
           
            // Extended arithmetic instructions
            0b10001 => self.execute_mno(), // MNO - Multiply
            0b10010 => self.execute_dzi(), // DZI - Divide
            0b10011 => self.execute_mod(), // MOD - Modulo
            
            // Invalid opcode
            _ => Err(MachineError::InvalidOpcode { opcode: self.kod }),
        }
    }
}
