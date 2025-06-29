//! operand resolution and value parsing

mod address_resolver;
mod number_parser;
mod register_parser;
mod validator;

pub use address_resolver::AddressResolver;
pub use number_parser::NumberParser;
pub use register_parser::RegisterParser;
pub use validator::Validator;

use crate::error::AssemblerError;
use crate::symbol_table::SymbolTable;
use parseid::ast::{AddressingMode, Operand};

pub struct OperandResolver {
    address_resolver: AddressResolver,
    number_parser: NumberParser,
    register_parser: RegisterParser,
    validator: Validator,
}

impl OperandResolver {
    pub fn new() -> Self {
        Self {
            address_resolver: AddressResolver::new(),
            number_parser: NumberParser::new(),
            register_parser: RegisterParser::new(),
            validator: Validator::new(),
        }
    }

    pub fn resolve_symbol_to_address(&self, operand: &Operand, symbol_table: &SymbolTable, current_address: u16, line: usize) -> Result<u16, AssemblerError> {
        match &operand.addressing_mode {
            AddressingMode::Direct => {
                self.address_resolver.resolve_direct(&operand.value, symbol_table, line, &self.number_parser, &self.validator)
            }
            AddressingMode::Immediate => {
                self.number_parser.parse_number(&operand.value, line)
            }
            AddressingMode::Indirect => {
                self.address_resolver.resolve_indirect(&operand.value, line, &self.number_parser, &self.validator)
            }
            AddressingMode::MultipleIndirect => {
                self.address_resolver.resolve_multiple_indirect(&operand.value, line, &self.number_parser, &self.validator)
            }
            AddressingMode::Register => {
                self.register_parser.parse_register(&operand.value, line)
            }
            AddressingMode::RegisterIndirect => {
                self.register_parser.parse_register(&operand.value, line)
            }
            AddressingMode::BaseRegister { base: _, offset } => {
                self.number_parser.parse_number(offset, line)
            }
            AddressingMode::Relative => {
                self.address_resolver.resolve_relative(&operand.value, current_address, line, &self.number_parser)
            }
            AddressingMode::Indexed { address, index: _ } => {
                self.address_resolver.resolve_indexed(address, symbol_table, line, &self.number_parser, &self.validator)
            }
        }
    }

    pub fn parse_signed_number(&self, value: &str, line: usize) -> Result<i32, AssemblerError> {
        self.number_parser.parse_signed_number(value, line)
    }
}

impl Default for OperandResolver {
    fn default() -> Self {
        Self::new()
    }
}
