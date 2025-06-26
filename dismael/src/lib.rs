//! Disassembler for Machine W binary code
//! Converts binary machine code back to readable Asmodeus assembly

use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DisassemblerError {
    #[error("Invalid opcode: {opcode} at address {address}")]
    InvalidOpcode { opcode: u8, address: u16 },
    #[error("Empty machine code")]
    EmptyCode,
    #[error("Address out of bounds: {address}")]
    AddressOutOfBounds { address: u16 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisassembledInstruction {
    pub address: u16,
    pub raw_value: u16,
    pub opcode: u8,
    pub argument: u16,
    pub mnemonic: String,
    pub operand: Option<String>,
    pub is_data: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddressingMode {
    None,
    Direct,
    Immediate,
    Indirect,
    Register,
    Relative,
}

pub struct Disassembler {
    /// jump targets for label generation
    jump_targets: HashSet<u16>,
    /// label names mapped to addresses
    labels: HashMap<u16, String>,
    /// data addresses (RST/RPA)
    data_addresses: HashSet<u16>,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            jump_targets: HashSet::new(),
            labels: HashMap::new(),
            data_addresses: HashSet::new(),
        }
    }

    /// Disassembles binary machine code into readable assembly
    /// 
    /// # Arguments
    /// * `machine_code` - Vector of 16-bit machine code words
    /// 
    /// # Returns
    /// * `Ok(Vec<String>)` - Vector of assembly code lines
    /// * `Err(DisassemblerError)` - If disassembly fails
    pub fn disassemble(&mut self, machine_code: &[u16]) -> Result<Vec<String>, DisassemblerError> {
        if machine_code.is_empty() {
            return Err(DisassemblerError::EmptyCode);
        }

        // first pass: find jump targets and data
        self.analyze_code(machine_code)?;

        // second pass
        self.generate_labels();

        // third pass: disassemble instructions
        let mut result = Vec::new();
        let mut i = 0;

        while i < machine_code.len() {
            let address = i as u16;
            let instruction = self.disassemble_instruction(address, machine_code[i])?;

            // add label if this address is a target
            if let Some(label) = self.labels.get(&address) {
                result.push(format!("{}:", label));
            }

            let line = self.format_instruction(&instruction);
            result.push(line);

            i += 1;
        }

        Ok(result)
    }

    fn analyze_code(&mut self, machine_code: &[u16]) -> Result<(), DisassemblerError> {
        for (i, &word) in machine_code.iter().enumerate() {
            let address = i as u16;
            let opcode = (word >> 11) & 0b11111;
            let argument = word & 0b0000011111111111;

            match opcode {
                0b00101 | 0b00110 | 0b10000 => {
                    // SOB (unconditional jump), SOM (conditional jump), or SOZ (conditional jump)
                    self.jump_targets.insert(argument);
                }
                0b00001..=0b00111 | 0b01000..=0b01111 | 0b10000 => {
                    // valid instruction opcodes - only mark as data if it looks like data access
                    if self.is_valid_address(argument) && 
                       self.could_be_data_reference(opcode as u8) &&
                       argument < machine_code.len() as u16 {
                        // only mark as data if its not a jump target and looks like actual data
                        if !self.jump_targets.contains(&argument) {
                            // target looks like an instruction or data
                            let target_word = machine_code[argument as usize];
                            let target_opcode = (target_word >> 11) & 0b11111;
                            if target_opcode > 0b10000 {
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


    fn generate_labels(&mut self) {
        // for jump targets
        for &address in &self.jump_targets {
            let label = format!("L_{:04X}", address);
            self.labels.insert(address, label);
        }

        // for data that might be referenced
        for &address in &self.data_addresses {
            if !self.labels.contains_key(&address) {
                let label = format!("DATA_{:04X}", address);
                self.labels.insert(address, label);
            }
        }
    }

    fn could_be_data_reference(&self, opcode: u8) -> bool {
        matches!(opcode, 
            0b00001 | // DOD
            0b00010 | // ODE  
            0b00011 | // ŁAD
            0b00100   // POB
        )
    }

    /// if an address is valid (within range)
    fn is_valid_address(&self, address: u16) -> bool {
        address < 2048
    }

    fn disassemble_instruction(&self, address: u16, word: u16) -> Result<DisassembledInstruction, DisassemblerError> {
        let opcode = (word >> 11) & 0b11111;
        let argument = word & 0b0000011111111111;

        let (mnemonic, operand, is_data) = match opcode {
            0b00001 => ("DOD".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b00010 => ("ODE".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b00011 => ("ŁAD".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b00100 => ("POB".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b00101 => ("SOB".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b00110 => ("SOM".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b10000 => ("SOZ".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b00111 => ("STP".to_string(), None, false),
            0b01000 => ("DNS".to_string(), None, false),
            0b01001 => ("PZS".to_string(), None, false),
            0b01010 => ("SDP".to_string(), None, false),
            0b01011 => ("CZM".to_string(), None, false),
            0b01100 => ("MSK".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b01101 => ("PWR".to_string(), None, false),
            0b01110 => ("WEJSCIE".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b01111 => ("WYJSCIE".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b01110 => ("WPR".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            0b01111 => ("WYJ".to_string(), Some(self.format_operand(argument, AddressingMode::Direct)), false),
            _ => {
                // unknown opcode - treat as data
                if self.data_addresses.contains(&address) {
                    ("RST".to_string(), Some(word.to_string()), true)
                } else {
                    return Err(DisassemblerError::InvalidOpcode { 
                        opcode: opcode as u8, 
                        address 
                    });
                }
            }
        };

        Ok(DisassembledInstruction {
            address,
            raw_value: word,
            opcode: opcode as u8,
            argument,
            mnemonic,
            operand,
            is_data,
            comment: None,
        })
    }

    /// based on its addressing mode
    fn format_operand(&self, argument: u16, mode: AddressingMode) -> String {
        match mode {
            AddressingMode::None => String::new(),
            AddressingMode::Direct => {
                if let Some(label) = self.labels.get(&argument) {
                    label.clone()
                } else {
                    argument.to_string()
                }
            }
            AddressingMode::Immediate => format!("#{}", argument),
            AddressingMode::Indirect => {
                if let Some(label) = self.labels.get(&argument) {
                    format!("[{}]", label)
                } else {
                    format!("[{}]", argument)
                }
            }
            AddressingMode::Register => format!("R{}", argument),
            AddressingMode::Relative => {
                if (argument & 0x400) != 0 {
                    // Negative (sign extend)
                    let offset = argument | 0xF800;
                    format!("{}", offset as i16)
                } else {
                    format!("+{}", argument)
                }
            }
        }
    }

    fn format_instruction(&self, instruction: &DisassembledInstruction) -> String {
        let mut result = String::new();
        
        // (without address comment for now - TODO)
        result.push_str("    ");
        result.push_str(&instruction.mnemonic);
        
        if let Some(ref operand) = instruction.operand {
            result.push(' ');
            result.push_str(operand);
        }

        if let Some(ref comment) = instruction.comment {
            result.push_str(" ; ");
            result.push_str(comment);
        }

        result
    }

    fn detect_addressing_mode(&self, opcode: u8, _argument: u16) -> AddressingMode {
        match opcode {
            0b00101 | 0b00110 | 0b10000 => AddressingMode::Direct, // jump instructions
            0b01100 => AddressingMode::Direct, // MSK
            0b01110 | 0b01111 => AddressingMode::Direct, // I/O
            _ => {
                // for other instructions, direct addressing
                // TODO: analyze patterns or have additional metadata

                AddressingMode::Direct
            }
        }
    }
}

impl Default for Disassembler {
    fn default() -> Self {
        Self::new()
    }
}

pub fn disassemble(machine_code: &[u16]) -> Result<Vec<String>, DisassemblerError> {
    let mut disassembler = Disassembler::new();
    disassembler.disassemble(machine_code)
}

pub fn disassemble_to_string(machine_code: &[u16]) -> Result<String, DisassemblerError> {
    let lines = disassemble(machine_code)?;
    Ok(lines.join("\n"))
}

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
                self.base.data_addresses.insert(addr);
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
