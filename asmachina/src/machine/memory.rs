use crate::error::MachineError;
use super::MachineW;

impl MachineW {
    pub fn read_memory(&self, address: u16) -> Result<u16, MachineError> {
        let addr = address & 0b0000011111111111; // limit to 11 bits (0-2047)
        if addr >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address: addr });
        }
        Ok(self.memory[addr as usize])
    }

    pub fn write_memory(&mut self, address: u16, value: u16) -> Result<(), MachineError> {
        let addr = address & 0b0000011111111111; // limit to 11 bits (0-2047)
        if addr >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address: addr });
        }
        self.memory[addr as usize] = value;
        Ok(())
    }

    /// loads program into memory (starting at address 0!)
    pub fn load_program(&mut self, program: &[u16]) -> Result<(), MachineError> {
        if program.len() > 2048 {
            return Err(MachineError::AddressOutOfBounds { 
                address: program.len() as u16 
            });
        }
        
        for (i, &instruction) in program.iter().enumerate() {
            self.memory[i] = instruction;
        }
        Ok(())
    }
}
