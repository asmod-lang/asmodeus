//! output formatting for disassembled instructions

use crate::types::DisassembledInstruction;
use std::collections::HashMap;

pub struct InstructionFormatter {
    labels: HashMap<u16, String>,
}

impl InstructionFormatter {
    pub fn new(labels: HashMap<u16, String>) -> Self {
        Self { labels }
    }

    pub fn format_instruction(&self, instruction: &DisassembledInstruction) -> String {
        let mut result = String::new();
        
        // for debugging and reference
        result.push_str(&format!("    ; {:04X}: {:04X}\n", instruction.address, instruction.raw_value));
        result.push_str("    ");
        result.push_str(&instruction.mnemonic);
        
        if let Some(ref operand) = instruction.operand {
            result.push(' ');
            result.push_str(operand);
        }

        if let Some(ref comment) = instruction.comment {
            result.push_str(" ; ");
            result.push_str(comment);
        }

        result
    }

    pub fn generate_labels(&self, jump_targets: &std::collections::HashSet<u16>, data_addresses: &std::collections::HashSet<u16>) -> HashMap<u16, String> {
        let mut labels = HashMap::new();

        // for jump targets
        for &address in jump_targets {
            let label = format!("L_{:04X}", address);
            labels.insert(address, label);
        }

        // for data that might be referenced
        for &address in data_addresses {
            if !labels.contains_key(&address) {
                let label = format!("DATA_{:04X}", address);
                labels.insert(address, label);
            }
        }

        labels
    }

    pub fn format_label(&self, address: u16) -> Option<String> {
        self.labels.get(&address).map(|label| format!("{}:", label))
    }
}
