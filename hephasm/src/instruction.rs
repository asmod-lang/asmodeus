//! instruction assembly and opcode mapping

use crate::error::AssemblerError;
use parseid::ast::{Instruction, AddressingMode};
use shared::{addressing_mode_bits, encode_instruction};

pub struct InstructionAssembler {
    extended_mode: bool,
}

impl InstructionAssembler {
    pub fn new() -> Self {
        Self {
            extended_mode: false,
        }
    }

    pub fn new_with_extended(extended_mode: bool) -> Self {
        Self {
            extended_mode,
        }
    }

    pub fn assemble_instruction(&self, instruction: &Instruction, argument: u16) -> Result<u16, AssemblerError> {
        let opcode = self.get_opcode(&instruction.opcode, instruction.line)?;
        
        let addressing_mode_bits = if let Some(operand) = &instruction.operand {
            match &operand.addressing_mode {
                AddressingMode::Direct => addressing_mode_bits::DIRECT,
                AddressingMode::Immediate => addressing_mode_bits::IMMEDIATE,
                AddressingMode::Indirect => addressing_mode_bits::INDIRECT,
                AddressingMode::MultipleIndirect => addressing_mode_bits::MULTIPLE_INDIRECT,
                AddressingMode::Register => addressing_mode_bits::REGISTER,
                AddressingMode::RegisterIndirect => addressing_mode_bits::REGISTER_INDIRECT,
                AddressingMode::BaseRegister { .. } => addressing_mode_bits::BASE_REGISTER,
                AddressingMode::Relative => addressing_mode_bits::RELATIVE,
                AddressingMode::Indexed { .. } => addressing_mode_bits::DIRECT,
            }
        } else {
            addressing_mode_bits::DIRECT
        };
        
        if argument > 255 {
            return Err(AssemblerError::AddressOutOfBounds {
                address: argument,
                line: instruction.line,
            });
        }

        Ok(encode_instruction(opcode, addressing_mode_bits, argument))
    }

    pub fn get_opcode(&self, instruction: &str, line: usize) -> Result<u8, AssemblerError> {
        match instruction.to_uppercase().as_str() {
            "DOD" => Ok(0b00001),
            "ODE" => Ok(0b00010),
            "ŁAD" | "LAD" => Ok(0b00011),
            "POB" => Ok(0b00100),
            "SOB" => Ok(0b00101),
            "SOM" => Ok(0b00110),
            "SOZ" => Ok(0b10000),
            "STP" => Ok(0b00111),
            "DNS" => Ok(0b01000),
            "PZS" => Ok(0b01001),
            "SDP" => Ok(0b01010),
            "CZM" => Ok(0b01011),
            "MSK" => Ok(0b01100),
            "PWR" => Ok(0b01101),
            "WPR" | "WEJSCIE" => Ok(0b01110),
            "WYJ" | "WYJSCIE" => Ok(0b01111),

            // extended opcodes
            "MNO" => {
                if self.extended_mode {
                    Ok(0b10001)
                } else {
                    Err(AssemblerError::ExtendedInstructionNotEnabled {
                        instruction: instruction.to_string(),
                        line,
                    })
                }
            },
            "DZI" => {
                if self.extended_mode {
                    Ok(0b10010)
                } else {
                    Err(AssemblerError::ExtendedInstructionNotEnabled {
                        instruction: instruction.to_string(),
                        line,
                    })
                }
            },
            "MOD" => {
                if self.extended_mode {
                    Ok(0b10011)
                } else {
                    Err(AssemblerError::ExtendedInstructionNotEnabled {
                        instruction: instruction.to_string(),
                        line,
                    })
                }
            },
            _ => Err(AssemblerError::InvalidOpcode {
                opcode: instruction.to_string(),
                line,
            }),
        }
    }
}

impl Default for InstructionAssembler {
    fn default() -> Self {
        Self::new()
    }
}
