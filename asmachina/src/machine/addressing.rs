use crate::error::MachineError;
use asmodeus_shared::{addressing_mode_bits, extract_addressing_mode};
use super::MachineW;

impl MachineW {
    /// resolves an operand address based on addressing mode
    pub(crate) fn resolve_effective_address(&self, instruction_code: u16) -> Result<u16, MachineError> {
        let addressing_mode_bits = extract_addressing_mode(instruction_code);
        
        match addressing_mode_bits {
            bits if bits == addressing_mode_bits::DIRECT => {
                Ok(self.ad)
            }
            bits if bits == addressing_mode_bits::IMMEDIATE => {
                Ok(self.ad)
            }
            bits if bits == addressing_mode_bits::INDIRECT => {
                if self.ad >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { address: self.ad });
                }
                let indirect_address = self.memory[self.ad as usize];
                if indirect_address >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { address: indirect_address });
                }
                Ok(indirect_address)
            }
            bits if bits == addressing_mode_bits::MULTIPLE_INDIRECT => {
                if self.ad >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { address: self.ad });
                }
                let first_indirect = self.memory[self.ad as usize];
                if first_indirect >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { address: first_indirect });
                }
                let second_indirect = self.memory[first_indirect as usize];
                if second_indirect >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { address: second_indirect });
                }
                Ok(second_indirect)
            }
            bits if bits == addressing_mode_bits::REGISTER => {
                let register_num = self.ad & 0b111;
                if register_num > 7 {
                    return Err(MachineError::InvalidRegister { register: register_num as u8 });
                }
                Ok(self.registers[register_num as usize])
            }
            bits if bits == addressing_mode_bits::REGISTER_INDIRECT => {
                let register_num = self.ad & 0b111;
                if register_num > 7 {
                    return Err(MachineError::InvalidRegister { register: register_num as u8 });
                }
                let address = self.registers[register_num as usize];
                if address >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { address });
                }
                Ok(address)
            }
            bits if bits == addressing_mode_bits::BASE_REGISTER => {
                let register_num = (self.ad >> 6) & 0b111;
                let offset = self.ad & 0b111111;
                if register_num > 7 {
                    return Err(MachineError::InvalidRegister { register: register_num as u8 });
                }
                let base_address = self.registers[register_num as usize];
                let effective_address = base_address.wrapping_add(offset);
                if effective_address >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { address: effective_address });
                }
                Ok(effective_address)
            }
            bits if bits == addressing_mode_bits::RELATIVE => {
                let offset = if (self.ad & 0x80) != 0 {
                    (self.ad | 0xFF00) as i16
                } else {
                    self.ad as i16
                };
                
                let target_address = (self.l as i32) + (offset as i32);
                if target_address < 0 || target_address >= 2048 {
                    return Err(MachineError::AddressOutOfBounds { 
                        address: target_address.max(0) as u16 
                    });
                }
                Ok(target_address as u16)
            }
            _ => {
                Err(MachineError::InvalidAddressingMode { mode: addressing_mode_bits })
            }
        }
    }
}
