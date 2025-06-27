//! Core parser implementation

use lexariel::{Token, TokenKind};
use crate::ast::{Program, ProgramElement};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;
use crate::instruction_parser::InstructionParser;
use crate::directive_parser::DirectiveParser;
use crate::macro_parser::MacroParser;

pub struct Parser {
    navigator: TokenNavigator,
}

impl Parser {
    /// creates a new parser with the given tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            navigator: TokenNavigator::new(tokens)
        }
    }

    /// parses the entire program
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::new();

        while let Some(token) = self.navigator.peek() {
            if token.kind == TokenKind::Eof {
                break;
            }

            let element = self.parse_program_element()?;
            program.add_element(element);
        }

        Ok(program)
    }

    /// parses a top-level program element
    fn parse_program_element(&mut self) -> Result<ProgramElement, ParserError> {
        Self::parse_program_element_with_navigator(&mut self.navigator)
    }

    /// parses a top-level program element using provided navigator
    pub(crate) fn parse_program_element_with_navigator(navigator: &mut TokenNavigator) -> Result<ProgramElement, ParserError> {
        match navigator.peek() {
            Some(token) => match &token.kind {
                TokenKind::LabelDef => {
                    let label = InstructionParser::parse_label_definition(navigator)?;
                    Ok(ProgramElement::LabelDefinition(label))
                }
                TokenKind::Directive => {
                    if token.value.to_uppercase() == "MAKRO" {
                        let macro_def = MacroParser::parse_macro_definition(navigator)?;
                        Ok(ProgramElement::MacroDefinition(macro_def))
                    } else {
                        let directive = DirectiveParser::parse_directive(navigator)?;
                        Ok(ProgramElement::Directive(directive))
                    }
                }
                TokenKind::Keyword => {
                    let instruction = InstructionParser::parse_instruction(navigator)?;
                    Ok(ProgramElement::Instruction(instruction))
                }
                TokenKind::Identifier => {
                    let macro_call = MacroParser::parse_macro_call(navigator)?;
                    Ok(ProgramElement::MacroCall(macro_call))
                }
                _ => Err(ParserError::UnexpectedToken {
                    line: token.line,
                    column: token.column,
                    expected: "instruction, directive, or label".to_string(),
                    found: format!("{}", token.kind),
                }),
            },
            None => Err(ParserError::UnexpectedEof {
                expected: "program element".to_string(),
            }),
        }
    }
}
