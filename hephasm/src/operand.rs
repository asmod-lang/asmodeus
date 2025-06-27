//! operand resolution and value parsing

use crate::error::AssemblerError;
use crate::symbol_table::SymbolTable;
use parseid::ast::{AddressingMode, Operand};

pub struct OperandResolver;

impl OperandResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_operand(&self, operand: &Operand, symbol_table: &SymbolTable, current_address: u16, line: usize) -> Result<u16, AssemblerError> {
        // try to resolve as symbol regardless of addressing mode
        if let Some(address) = symbol_table.get_address(&operand.value) {
            return Ok(address);
        }

        match &operand.addressing_mode {
            AddressingMode::Immediate => {
                // immediate addressing, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AddressingMode::Direct => {
                // direct addressing, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AddressingMode::Indirect => {
                // indirect addressing, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AddressingMode::Register => {
                // register addressing, extract register number
                self.parse_register(&operand.value, line)
            }
            AddressingMode::Relative => {
                // relative addressing, calculate offset from current position
                let offset = self.parse_signed_number(&operand.value, line)?;
                let target = (current_address as i32) + offset;
                if target < 0 || target > 2047 {
                    return Err(AssemblerError::AddressOutOfBounds {
                        address: target as u16,
                        line,
                    });
                }
                Ok(target as u16)
            }
            AddressingMode::Indexed { address, index: _ } => {
                // indexed addressing, try to resolve base address as symbol first
                if let Some(addr) = symbol_table.get_address(address) {
                    Ok(addr)
                } else {
                    match self.parse_number(address, line) {
                        Ok(num) => Ok(num),
                        Err(_) if self.is_identifier(address) => {
                            Err(AssemblerError::UndefinedSymbol {
                                symbol: address.clone(),
                                line,
                            })
                        }
                        Err(e) => Err(e),
                    }
                }
            }
            _ => {
                // other addressing modes, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub fn is_identifier(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let first_char = s.chars().next().unwrap();
        (first_char.is_alphabetic() || first_char == '_') && 
        !s.starts_with("0x") && !s.starts_with("0X") && 
        !s.starts_with("0b") && !s.starts_with("0B") &&
        !s.chars().all(|c| c.is_ascii_digit())
    }

    pub fn parse_number(&self, value: &str, line: usize) -> Result<u16, AssemblerError> {
        if value.starts_with("0x") || value.starts_with("0X") {
            // hex
            u16::from_str_radix(&value[2..], 16)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else if value.starts_with("0b") || value.starts_with("0B") {
            // binary
            u16::from_str_radix(&value[2..], 2)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else {
            // decimal
            value.parse::<u16>()
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        }
    }

    /// relative addressing
    pub fn parse_signed_number(&self, value: &str, line: usize) -> Result<i32, AssemblerError> {
        let mut is_negative = false;
        let mut num_str = value;
        
        // negative sign
        if value.starts_with('-') {
            is_negative = true;
            num_str = &value[1..];
        }
        
        let result = if num_str.starts_with("0x") || num_str.starts_with("0X") {
            // hex
            i32::from_str_radix(&num_str[2..], 16)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else if num_str.starts_with("0b") || num_str.starts_with("0B") {
            // binary
            i32::from_str_radix(&num_str[2..], 2)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else {
            // decimal
            num_str.parse::<i32>()
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        }?;
        
        Ok(if is_negative { -result } else { result })
    }

    pub fn parse_register(&self, value: &str, line: usize) -> Result<u16, AssemblerError> {
        if !value.to_uppercase().starts_with('R') {
            return Err(AssemblerError::InvalidNumber {
                value: value.to_string(),
                line,
            });
        }

        let reg_num = &value[1..];
        reg_num.parse::<u16>()
            .map_err(|_| AssemblerError::InvalidNumber {
                value: value.to_string(),
                line,
            })
    }
}

impl Default for OperandResolver {
    fn default() -> Self {
        Self::new()
    }
}
