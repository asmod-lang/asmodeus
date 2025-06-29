//! third pass: code generation

use crate::error::AssemblerError;
use crate::symbol_table::SymbolTable;
use crate::instruction::InstructionAssembler;
use crate::operand::OperandResolver;
use crate::directive::DirectiveProcessor;
use parseid::ast::ProgramElement;

pub struct ThirdPass;

impl ThirdPass {
    pub fn execute(
        memory: &mut Vec<u16>,
        current_address: &mut u16,
        symbol_table: &SymbolTable,
        instruction_assembler: &InstructionAssembler,
        operand_resolver: &OperandResolver,
        directive_processor: &DirectiveProcessor,
        elements: &[ProgramElement]
    ) -> Result<(), AssemblerError> {
        *current_address = 0;

        for element in elements {
            match element {
                ProgramElement::Instruction(inst) => {
                    let argument = if let Some(operand) = &inst.operand {
                        operand_resolver.resolve_symbol_to_address(operand, symbol_table, *current_address, inst.line)?
                    } else {
                        0
                    };

                    let machine_code = instruction_assembler.assemble_instruction(inst, argument)?;
                    memory[*current_address as usize] = machine_code;
                    *current_address += 1;
                }
                ProgramElement::Directive(dir) => {
                    directive_processor.assemble_directive(dir, memory, *current_address as usize)?;
                    match dir.name.to_uppercase().as_str() {
                        "RST" | "RPA" => {
                            *current_address += 1;
                        }
                        _ => {}
                    }
                }
                ProgramElement::LabelDefinition(_) => {
                    // labels dont generate code
                }
                _ => {}
            }
        }

        Ok(())
    }
}
