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
            match opcode {
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

    pub fn _detect_addressing_mode(&self, opcode: u8, _argument: u16) -> AddressingMode {
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
}
