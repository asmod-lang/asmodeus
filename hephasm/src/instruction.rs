//! instruction assembly and opcode mapping

use crate::error::AssemblerError;
use parseid::ast::Instruction;

pub struct InstructionAssembler;

impl InstructionAssembler {
    pub fn new() -> Self {
        Self
    }

    pub fn assemble_instruction(&self, instruction: &Instruction, argument: u16) -> Result<u16, AssemblerError> {
        let opcode = self.get_opcode(&instruction.opcode, instruction.line)?;
        
        // ensure argument fits in 11 bits
        if argument > 2047 {
            return Err(AssemblerError::AddressOutOfBounds {
                address: argument,
                line: instruction.line,
            });
        }

        // combine opcode (5 bits) and argument (11 bits)
        let machine_code = ((opcode as u16) << 11) | (argument & 0x07FF);
        Ok(machine_code)
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
