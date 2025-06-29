use crate::error::MachineError;
use crate::machine::MachineW;
use shared::{extract_addressing_mode, addressing_mode_bits};

impl MachineW {
    /// MNO - Multiply: (AK) * ((AD)) → AK
    pub(crate) fn execute_mno(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let addressing_mode_bits = extract_addressing_mode(raw_instruction);
        
        let operand = if addressing_mode_bits == addressing_mode_bits::IMMEDIATE {
            self.ad
        } else {
            let effective_address = self.resolve_effective_address(raw_instruction)?;
            self.read_memory(effective_address)?
        };
        
        // 16-bit multiplication
        let result = (self.ak as u32) * (operand as u32);
        
        // only lower 16 bits in AK (overflow is ignored)
        self.ak = (result & 0xFFFF) as u16;
        
        Ok(())
    }

    /// DZI - Divide: (AK) / ((AD)) → AK
    pub(crate) fn execute_dzi(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let addressing_mode_bits = extract_addressing_mode(raw_instruction);
        
        let operand = if addressing_mode_bits == addressing_mode_bits::IMMEDIATE {
            self.ad
        } else {
            let effective_address = self.resolve_effective_address(raw_instruction)?;
            self.read_memory(effective_address)?
        };
        
        if operand == 0 {
            return Err(MachineError::DivisionByZero { 
                address: self.l.wrapping_sub(1) 
            });
        }
        
        self.ak = self.ak / operand;
        Ok(())
    }

    /// MOD - Modulo: (AK) % ((AD)) → AK
    pub(crate) fn execute_mod(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let addressing_mode_bits = extract_addressing_mode(raw_instruction);
        
        let operand = if addressing_mode_bits == addressing_mode_bits::IMMEDIATE {
            self.ad
        } else {
            let effective_address = self.resolve_effective_address(raw_instruction)?;
            self.read_memory(effective_address)?
        };
        
        if operand == 0 {
            return Err(MachineError::DivisionByZero { 
                address: self.l.wrapping_sub(1) 
            });
        }
        
        self.ak = self.ak % operand;
        Ok(())
    }
}
