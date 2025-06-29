//! address resolution for different addressing modes

use crate::error::AssemblerError;
use crate::symbol_table::SymbolTable;
use super::{NumberParser, Validator};

pub struct AddressResolver;

impl AddressResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_direct(
        &self, 
        value: &str, 
        symbol_table: &SymbolTable, 
        line: usize, 
        number_parser: &NumberParser, 
        validator: &Validator
    ) -> Result<u16, AssemblerError> {
        // direct addressing, try to resolve symbol first, then parse as number
        if let Some(addr) = symbol_table.get_address(value) {
            Ok(addr)
        } else {
            match number_parser.parse_number(value, line) {
                Ok(num) => Ok(num),
                Err(_) if validator.is_identifier(value) => {
                    Err(AssemblerError::UndefinedSymbol {
                        symbol: value.to_string(),
                        line,
                    })
                }
                Err(e) => Err(e),
            }
        }
    }

    pub fn resolve_indirect(
        &self, 
        value: &str, 
        line: usize, 
        number_parser: &NumberParser, 
        validator: &Validator
    ) -> Result<u16, AssemblerError> {
        // indirect addressing, parse as number or fail with undefined symbol
        match number_parser.parse_number(value, line) {
            Ok(num) => Ok(num),
            Err(_) if validator.is_identifier(value) => {
                Err(AssemblerError::UndefinedSymbol {
                    symbol: value.to_string(),
                    line,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn resolve_multiple_indirect(
        &self, 
        value: &str, 
        line: usize, 
        number_parser: &NumberParser, 
        validator: &Validator
    ) -> Result<u16, AssemblerError> {
        // multiple indirect addressing, parse as number or fail with undefined symbol
        match number_parser.parse_number(value, line) {
            Ok(num) => Ok(num),
            Err(_) if validator.is_identifier(value) => {
                Err(AssemblerError::UndefinedSymbol {
                    symbol: value.to_string(),
                    line,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn resolve_relative(
        &self, 
        value: &str, 
        current_address: u16, 
        line: usize, 
        number_parser: &NumberParser
    ) -> Result<u16, AssemblerError> {
        // relative addressing, calculate offset from current position
        let offset = number_parser.parse_signed_number(value, line)?;
        let target = (current_address as i32) + offset;
        if target < 0 || target > 2047 {
            return Err(AssemblerError::AddressOutOfBounds {
                address: target as u16,
                line,
            });
        }
        Ok(target as u16)
    }

    pub fn resolve_indexed(
        &self, 
        address: &str, 
        symbol_table: &SymbolTable, 
        line: usize, 
        number_parser: &NumberParser, 
        validator: &Validator
    ) -> Result<u16, AssemblerError> {
        // indexed addressing, try to resolve base address as symbol first
        if let Some(addr) = symbol_table.get_address(address) {
            Ok(addr)
        } else {
            match number_parser.parse_number(address, line) {
                Ok(num) => Ok(num),
                Err(_) if validator.is_identifier(address) => {
                    Err(AssemblerError::UndefinedSymbol {
                        symbol: address.to_string(),
                        line,
                    })
                }
                Err(e) => Err(e),
            }
        }
    }
}

impl Default for AddressResolver {
    fn default() -> Self {
        Self::new()
    }
}
