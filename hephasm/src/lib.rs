//! Hephasm - Assembler for Asmodeus assembly language
//! Converts AST into binary machine code for Machine W (asmachine)

mod error;
mod types;
mod symbol_table;
mod macro_processor;
mod instruction;
mod operand;
mod directive;
mod core;

pub use error::AssemblerError;
pub use types::{Symbol, SymbolType, ExpandedMacro};
pub use symbol_table::SymbolTable;
pub use core::Assembler;

use parseid::ast::Program;

pub fn assemble_source(source: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let program = parseid::parse_source(source)?;
    let mut assembler = Assembler::new();
    Ok(assembler.assemble(&program)?)
}

pub fn assemble_program(program: &Program) -> Result<Vec<u16>, AssemblerError> {
    let mut assembler = Assembler::new();
    assembler.assemble(program)
}
