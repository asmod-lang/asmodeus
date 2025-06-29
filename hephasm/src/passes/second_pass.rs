//! second pass: symbol table building

use crate::error::AssemblerError;
use crate::symbol_table::SymbolTable;
use crate::types::SymbolType;
use parseid::ast::ProgramElement;

pub struct SecondPass;

impl SecondPass {
    pub fn execute(
        symbol_table: &mut SymbolTable,
        current_address: &mut u16,
        elements: &[ProgramElement]
    ) -> Result<(), AssemblerError> {
        *current_address = 0;

        for element in elements {
            match element {
                ProgramElement::LabelDefinition(label) => {
                    symbol_table.define(
                        label.name.clone(),
                        *current_address,
                        SymbolType::Label,
                    ).map_err(|mut e| {
                        if let AssemblerError::DuplicateSymbol { line, .. } = &mut e {
                            *line = label.line;
                        }
                        e
                    })?;
                }
                ProgramElement::Instruction(_) => {
                    *current_address += 1;
                    if *current_address > 2048 {
                        return Err(AssemblerError::MemoryOverflow);
                    }
                }
                ProgramElement::Directive(dir) => {
                    match dir.name.to_uppercase().as_str() {
                        "RST" | "RPA" => {
                            *current_address += 1;
                        }
                        _ => {}
                    }
                    if *current_address > 2048 {
                        return Err(AssemblerError::MemoryOverflow);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
