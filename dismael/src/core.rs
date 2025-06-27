//! core disassembler implementation

use crate::error::DisassemblerError;
use crate::analyzer::CodeAnalyzer;
use crate::formatter::InstructionFormatter;
use crate::instruction::InstructionDecoder;
use std::collections::HashMap;

pub struct Disassembler {
    analyzer: CodeAnalyzer,
    labels: HashMap<u16, String>,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            analyzer: CodeAnalyzer::new(),
            labels: HashMap::new(),
        }
    }

    /// disassembles binary machine code into readable assembly
    pub fn disassemble(&mut self, machine_code: &[u16]) -> Result<Vec<String>, DisassemblerError> {
        if machine_code.is_empty() {
            return Err(DisassemblerError::EmptyCode);
        }

        // first pass: find jump targets and data
        self.analyzer.analyze_code(machine_code)?;

        // second pass: generate labels
        let formatter = InstructionFormatter::new(HashMap::new());
        self.labels = formatter.generate_labels(
            self.analyzer.get_jump_targets(), 
            self.analyzer.get_data_addresses()
        );

        // third pass: disassemble instructions
        let decoder = InstructionDecoder::new(self.labels.clone());
        let output_formatter = InstructionFormatter::new(self.labels.clone());
        let mut result = Vec::new();
        let mut i = 0;

        while i < machine_code.len() {
            let address = i as u16;
            let is_data = self.analyzer.is_data_address(address);
            let instruction = decoder.disassemble_instruction(address, machine_code[i], is_data)?;

            // add label if this address is a target
            if let Some(label_line) = output_formatter.format_label(address) {
                result.push(label_line);
            }

            let line = output_formatter.format_instruction(&instruction);
            result.push(line);

            i += 1;
        }

        Ok(result)
    }

    // public access methods for analyzer
    pub fn get_analyzer_mut(&mut self) -> &mut CodeAnalyzer {
        &mut self.analyzer
    }

    pub fn get_analyzer(&self) -> &CodeAnalyzer {
        &self.analyzer
    }
}

impl Default for Disassembler {
    fn default() -> Self {
        Self::new()
    }
}
