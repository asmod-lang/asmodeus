use crate::error::DisassemblerError;
use std::collections::HashSet;

pub struct CodeAnalyzer {
    jump_targets: HashSet<u16>,
    data_addresses: HashSet<u16>,
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        Self {
            jump_targets: HashSet::new(),
            data_addresses: HashSet::new(),
        }
    }

    pub fn analyze_code(&mut self, machine_code: &[u16]) -> Result<(), DisassemblerError> {
        for (i, &word) in machine_code.iter().enumerate() {
            let address = i as u16;
            let opcode = (word >> 11) & 0b11111;
            let argument = word & 0b0000011111111111;

            match opcode {
                0b00101 | 0b00110 | 0b10000 => {
                    // SOB (unconditional jump), SOM (conditional jump), or SOZ (conditional jump)
                    self.jump_targets.insert(argument);
                }
                0b00001..=0b00111 | 0b01000..=0b01111 | 0b10001..=0b10011 => {
                    // valid instruction opcodes (including extended set)
                    // only mark as data if it looks like data access
                    if self.is_valid_address(argument) && 
                       self.could_be_data_reference(opcode as u8) &&
                       argument < machine_code.len() as u16 {
                        // only mark as data if its not a jump target and looks like actual data
                        if !self.jump_targets.contains(&argument) {
                            // target looks like an instruction or data
                            let target_word = machine_code[argument as usize];
                            let target_opcode = (target_word >> 11) & 0b11111;
                            if target_opcode > 0b10011 {
                                // invalid opcode, likely data
                                self.data_addresses.insert(argument);
                            }
                        }
                    }
                }
                _ => {
                    // invalid opcode - mark as data
                    self.data_addresses.insert(address);
                }
            }
        }

        Ok(())
    }

    pub fn could_be_data_reference(&self, opcode: u8) -> bool {
        matches!(opcode, 0b00001..=0b00100 | 0b01100 | 0b01110..=0b01111 | 0b10001..=0b10011)
    }

    pub fn is_valid_address(&self, address: u16) -> bool {
        address < 2048
    }

    pub fn get_jump_targets(&self) -> &HashSet<u16> {
        &self.jump_targets
    }

    pub fn get_data_addresses(&self) -> &HashSet<u16> {
        &self.data_addresses
    }

    pub fn is_data_address(&self, address: u16) -> bool {
        self.data_addresses.contains(&address)
    }

    pub fn add_data_address(&mut self, address: u16) {
        self.data_addresses.insert(address);
    }
}
