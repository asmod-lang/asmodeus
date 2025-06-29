//! directive processing (RST, RPA, etc.)

use crate::error::AssemblerError;
use crate::operand::OperandResolver;
use parseid::ast::Directive;

pub struct DirectiveProcessor {
    operand_resolver: OperandResolver,
}

impl DirectiveProcessor {
    pub fn new() -> Self {
        Self {
            operand_resolver: OperandResolver::new(),
        }
    }

    pub fn assemble_directive(&self, directive: &Directive, memory: &mut [u16], current_address: usize) -> Result<(), AssemblerError> {
        match directive.name.to_uppercase().as_str() {
            "RST" => {
                let value = if directive.arguments.is_empty() {
                    0
                } else {
                    let signed_value = self.operand_resolver.parse_signed_number(&directive.arguments[0], directive.line)?;
                    signed_value as u16
                };
                memory[current_address] = value;
            }
            "RPA" => {
                memory[current_address] = 0;
            }
            "MAKRO" | "KONM" | "NAZWA_LOKALNA" => {
                // handled in macro processing
                // valid but dont produce machine code
            }
            _ => {
                // unknown directive
                return Err(AssemblerError::InvalidOpcode {
                    opcode: directive.name.clone(),
                    line: directive.line,
                });
            }
        }
        Ok(())
    }
}

impl Default for DirectiveProcessor {
    fn default() -> Self {
        Self::new()
    }
}
