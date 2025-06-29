//! core assembler implementation (3-pass assembly process)

use crate::error::AssemblerError;
use crate::symbol_table::SymbolTable;
use crate::macro_processor::MacroProcessor;
use crate::instruction::InstructionAssembler;
use crate::operand::OperandResolver;
use crate::directive::DirectiveProcessor;
use crate::types::SymbolType;
use parseid::ast::*;

pub struct Assembler {
    symbol_table: SymbolTable,
    macro_processor: MacroProcessor,
    instruction_assembler: InstructionAssembler,
    operand_resolver: OperandResolver,
    directive_processor: DirectiveProcessor,
    memory: Vec<u16>,
    current_address: u16,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            macro_processor: MacroProcessor::new(),
            instruction_assembler: InstructionAssembler::new(),
            operand_resolver: OperandResolver::new(),
            directive_processor: DirectiveProcessor::new(),
            memory: vec![0; 2048],
            current_address: 0,
        }
    }

    pub fn assemble(&mut self, program: &Program) -> Result<Vec<u16>, AssemblerError> {
        self.reset();

        // first pass: collect macro definitions and expand macro calls
        let expanded_program = self.macro_processor.expand_macros(program)?;

        // second pass: build symbol table
        self.build_symbol_table(&expanded_program)?;

        // third pass: generate machine code
        self.current_address = 0;
        self.generate_code(&expanded_program)?;

        // return only the used portion of memory
        let used_memory = self.current_address as usize;
        Ok(self.memory[0..used_memory].to_vec())
    }

    fn reset(&mut self) {
        self.symbol_table.clear();
        self.macro_processor.clear();
        self.memory.fill(0);
        self.current_address = 0;
    }

    /// Second pass: build symbol table
    fn build_symbol_table(&mut self, elements: &[ProgramElement]) -> Result<(), AssemblerError> {
        self.current_address = 0;

        for element in elements {
            match element {
                ProgramElement::LabelDefinition(label) => {
                    self.symbol_table.define(
                        label.name.clone(),
                        self.current_address,
                        SymbolType::Label,
                    ).map_err(|mut e| {
                        if let AssemblerError::DuplicateSymbol { line, .. } = &mut e {
                            *line = label.line;
                        }
                        e
                    })?;
                }
                ProgramElement::Instruction(_) => {
                    self.current_address += 1;
                    if self.current_address > 2048 {
                        return Err(AssemblerError::MemoryOverflow);
                    }
                }
                ProgramElement::Directive(dir) => {
                    match dir.name.to_uppercase().as_str() {
                        "RST" | "RPA" => {
                            self.current_address += 1;
                        }
                        _ => {}
                    }
                    if self.current_address > 2048 {
                        return Err(AssemblerError::MemoryOverflow);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Third pass: generate machine code
    fn generate_code(&mut self, elements: &[ProgramElement]) -> Result<(), AssemblerError> {
        self.current_address = 0;

        for element in elements {
            match element {
                ProgramElement::Instruction(inst) => {
                    let argument = if let Some(operand) = &inst.operand {
                        self.operand_resolver.resolve_symbol_to_address(operand, &self.symbol_table, self.current_address, inst.line)?
                    } else {
                        0
                    };

                    let machine_code = self.instruction_assembler.assemble_instruction(inst, argument)?;
                    self.memory[self.current_address as usize] = machine_code;
                    self.current_address += 1;
                }
                ProgramElement::Directive(dir) => {
                    self.directive_processor.assemble_directive(dir, &mut self.memory, self.current_address as usize)?;
                    match dir.name.to_uppercase().as_str() {
                        "RST" | "RPA" => {
                            self.current_address += 1;
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

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}
