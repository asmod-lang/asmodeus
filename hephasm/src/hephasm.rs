//! Main assembler struct and public interface

use crate::error::AssemblerError;
use crate::symbol_table::SymbolTable;
use crate::macro_processor::MacroProcessor;
use crate::instruction::InstructionAssembler;
use crate::operand::OperandResolver;
use crate::directive::DirectiveProcessor;
use crate::passes::{FirstPass, SecondPass, ThirdPass};
use parseid::ast::Program;

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

    pub fn new_with_extended(extended_mode: bool) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            macro_processor: MacroProcessor::new(),
            instruction_assembler: InstructionAssembler::new_with_extended(extended_mode),
            operand_resolver: OperandResolver::new(),
            directive_processor: DirectiveProcessor::new(),
            memory: vec![0; 2048],
            current_address: 0,
        }
    }

    pub fn assemble(&mut self, program: &Program) -> Result<Vec<u16>, AssemblerError> {
        self.reset();

        // first pass: collect macro definitions and expand macro calls
        let expanded_program = FirstPass::execute(&mut self.macro_processor, program)?;

        // second pass: build symbol table
        SecondPass::execute(&mut self.symbol_table, &mut self.current_address, &expanded_program)?;

        // third pass: generate machine code
        self.current_address = 0;
        ThirdPass::execute(
            &mut self.memory,
            &mut self.current_address,
            &self.symbol_table,
            &self.instruction_assembler,
            &self.operand_resolver,
            &self.directive_processor,
            &expanded_program
        )?;

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
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}
