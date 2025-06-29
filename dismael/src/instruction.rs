//! instruction decoding logic for individual instructions

use crate::error::DisassemblerError;
use crate::types::{DisassembledInstruction, AddressingMode};
use std::collections::HashMap;

pub struct InstructionDecoder {
    labels: HashMap<u16, String>,
}

impl InstructionDecoder {
    pub fn new(labels: HashMap<u16, String>) -> Self {
        Self { labels }
    }

    pub fn disassemble_instruction(&self, address: u16, word: u16, is_data: bool) -> Result<DisassembledInstruction, DisassemblerError> {
        let opcode = (word >> 11) & 0b11111;
        let argument = word & 0b0000011111111111;

        let (mnemonic, operand, is_data_result) = if is_data {
            ("RST".to_string(), Some(word.to_string()), true)
        } else {
            let addressing_mode = self.detect_addressing_mode(word);
            
            match opcode {
                0b00001 => ("DOD".to_string(), Some(self.format_operand(argument, addressing_mode)), false),
                0b00010 => ("ODE".to_string(), Some(self.format_operand(argument, addressing_mode)), false),
                0b00011 => ("ŁAD".to_string(), Some(self.format_operand(argument, addressing_mode)), false),
                0b00100 => ("POB".to_string(), Some(self.format_operand(argument, addressing_mode)), false),
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
                _ => {
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
            is_data: is_data_result,
            comment: None,
        })
    }

    pub fn detect_addressing_mode(&self, instruction: u16) -> AddressingMode {
        use shared::{extract_addressing_mode, addressing_mode_bits};
        
        let mode_bits = extract_addressing_mode(instruction);
        
        match mode_bits {
            bits if bits == addressing_mode_bits::DIRECT => AddressingMode::Direct,
            bits if bits == addressing_mode_bits::IMMEDIATE => AddressingMode::Immediate,
            bits if bits == addressing_mode_bits::INDIRECT => AddressingMode::Indirect,
            bits if bits == addressing_mode_bits::REGISTER => AddressingMode::Register,
            bits if bits == addressing_mode_bits::RELATIVE => AddressingMode::Relative,
            // default to Direct
            _ => AddressingMode::Direct,
        }
    }

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
                    // negative (sign extend)
                    let offset = argument | 0xF800;
                    format!("{}", offset as i16)
                } else {
                    format!("+{}", argument)
                }
            }
        }
    }
}
