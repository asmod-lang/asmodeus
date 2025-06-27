//! Macro parsing functionality

use lexariel::TokenKind;
use crate::ast::{MacroDefinition, MacroCall, ProgramElement};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;
use crate::parser::Parser;

/// Parses macro definitions and calls
pub(crate) struct MacroParser;

impl MacroParser {
    pub fn parse_macro_definition(navigator: &mut TokenNavigator) -> Result<MacroDefinition, ParserError> {
        let makro_token = navigator.consume(TokenKind::Directive, "MAKRO")?;
        let line = makro_token.line;
        let column = makro_token.column;

        let name_token = navigator.consume(TokenKind::Identifier, "macro name")?;
        let name = name_token.value;

        let mut parameters = Vec::new();
        let mut body = Vec::new();

        // parameters (optional)
        while let Some(token) = navigator.peek() {
            match token.kind {
                TokenKind::Identifier => {
                    parameters.push(navigator.advance().unwrap().value.clone());
                }
                TokenKind::Directive if token.value.to_uppercase() == "KONM" => {
                    break;
                }
                TokenKind::Keyword | TokenKind::LabelDef => {
                    break;
                }
                _ => {
                    navigator.advance(); // skip other tokens
                }
            }
        }

        // until KONM
        while let Some(token) = navigator.peek() {
            if token.kind == TokenKind::Directive && token.value.to_uppercase() == "KONM" {
                navigator.advance(); // consume KONM
                break;
            }
            if token.kind == TokenKind::Eof {
                return Err(ParserError::InvalidMacroDefinition {
                    line,
                    column,
                    message: "Missing KONM to end macro definition".to_string(),
                });
            }
            body.push(Parser::parse_program_element_with_navigator(navigator)?);
        }

        Ok(MacroDefinition {
            name,
            parameters,
            body,
            line,
            column,
        })
    }

    pub fn parse_macro_call(navigator: &mut TokenNavigator) -> Result<MacroCall, ParserError> {
        let token = navigator.consume(TokenKind::Identifier, "macro name")?;
        let name = token.value;
        let line = token.line;
        let column = token.column;

        let mut arguments = Vec::new();

        // arguments
        while let Some(arg_token) = navigator.peek() {
            match arg_token.kind {
                TokenKind::Number | TokenKind::Identifier => {
                    arguments.push(navigator.advance().unwrap().value.clone());
                }
                TokenKind::Eof | TokenKind::LabelDef | TokenKind::Keyword | TokenKind::Directive => {
                    break;
                }
                _ => {
                    navigator.advance(); // skip other tokens
                }
            }
        }

        Ok(MacroCall {
            name,
            arguments,
            line,
            column,
        })
    }
}
