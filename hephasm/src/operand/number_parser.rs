//! number parsing utilities for different formats

use crate::error::AssemblerError;

pub struct NumberParser;

impl NumberParser {
    pub fn new() -> Self {
        Self
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

    /// for relative addressing
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
}

impl Default for NumberParser {
    fn default() -> Self {
        Self::new()
    }
}
