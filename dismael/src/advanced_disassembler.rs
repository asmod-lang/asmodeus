//! disassembler with flow analysis capabilities

use crate::error::DisassemblerError;
use crate::core::Disassembler;
use std::collections::HashSet;

pub struct AdvancedDisassembler {
    base: Disassembler,
    /// function entry points
    functions: HashSet<u16>,
    /// code vs data separation
    code_regions: Vec<(u16, u16)>,
}

impl AdvancedDisassembler {
    pub fn new() -> Self {
        Self {
            base: Disassembler::new(),
            functions: HashSet::new(),
            code_regions: Vec::new(),
        }
    }

    pub fn analyze_flow(&mut self, machine_code: &[u16]) -> Result<(), DisassemblerError> {
        // entry point at 0
        let mut to_analyze = vec![0u16];
        let mut analyzed = HashSet::new();
        let mut code_addresses = HashSet::new();

        while let Some(address) = to_analyze.pop() {
            if analyzed.contains(&address) || address as usize >= machine_code.len() {
                continue;
            }

            analyzed.insert(address);
            code_addresses.insert(address);

            let word = machine_code[address as usize];
            let opcode = (word >> 11) & 0b11111;
            let argument = word & 0b0000011111111111;

            match opcode {
                0b00101 => {
                    // SOB - unconditional jump
                    if (argument as usize) < machine_code.len() {
                        to_analyze.push(argument);
                    }
                    // dont continue to next instruction after unconditional jump
                }
                0b00110 => {
                    // SOM - conditional jump
                    if (argument as usize) < machine_code.len() {
                        to_analyze.push(argument);
                    }
                    // continue to next instruction too
                    to_analyze.push(address + 1);
                }
                0b00111 => {
                    // STP - stop
                }
                0b01101 => {
                    // PWR - return from interrupt dont continue normally
                }
                0b00001..=0b00110 | 0b01000..=0b01100 | 0b01110..=0b01111 => {
                    // other valid instructions - continue to next
                    to_analyze.push(address + 1);
                }
                _ => {
                    // invalid opcode - might be data, dont continue
                }
            }
        }

        // mark non-code addresses as data
        for i in 0..machine_code.len() {
            let addr = i as u16;
            if !code_addresses.contains(&addr) {
                self.base.get_analyzer_mut().add_data_address(addr);
            }
        }

        Ok(())
    }

    pub fn disassemble(&mut self, machine_code: &[u16]) -> Result<Vec<String>, DisassemblerError> {
        self.analyze_flow(machine_code)?;
        self.base.disassemble(machine_code)
    }
}

impl Default for AdvancedDisassembler {
    fn default() -> Self {
        Self::new()
    }
}
