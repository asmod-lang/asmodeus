//! Instruction parsing functionality

use lexariel::TokenKind;
use crate::ast::{Instruction, LabelDefinition};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;
use crate::operand_parser::OperandParser;

/// Parses instructions and labels
pub(crate) struct InstructionParser;

impl InstructionParser {
    pub fn parse_label_definition(navigator: &mut TokenNavigator) -> Result<LabelDefinition, ParserError> {
        let token = navigator.consume(TokenKind::LabelDef, "label definition")?;
        Ok(LabelDefinition {
            name: token.value,
            line: token.line,
            column: token.column,
        })
    }

    pub fn parse_instruction(navigator: &mut TokenNavigator) -> Result<Instruction, ParserError> {
        let token = navigator.consume(TokenKind::Keyword, "instruction")?;
        let opcode = token.value.clone();
        let line = token.line;
        let column = token.column;

        let operand = if Self::has_operand(&opcode) {
            Some(OperandParser::parse_operand(navigator)?)
        } else {
            None
        };

        Ok(Instruction {
            opcode,
            operand,
            line,
            column,
        })
    }

    fn has_operand(opcode: &str) -> bool {
        match opcode.to_uppercase().as_str() {
            "STP" | "DNS" | "PZS" | "SDP" | "CZM" | "PWR" | "WPR" | "WYJ" | "WEJSCIE" | "WYJSCIE" => false,
            _ => true,
        }
    }
}
