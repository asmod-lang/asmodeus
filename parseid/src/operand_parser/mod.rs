mod immediate_parser;
mod indirect_parser;
mod register_parser;
mod relative_parser;
mod direct_parser;
mod validator;

use lexariel::TokenKind;
use crate::ast::{Operand};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;
use immediate_parser::ImmediateParser;
use indirect_parser::IndirectParser;
use register_parser::RegisterParser;
use relative_parser::RelativeParser;
use direct_parser::DirectParser;
use validator::OperandValidator;

pub(crate) struct OperandParser;

impl OperandParser {
    /// with addressing mode
    pub fn parse_operand(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        OperandValidator::validate_has_operand(navigator)?;

        let token = navigator.peek().unwrap();

        match &token.kind {
            TokenKind::Punctuation if token.value == "#" => {
                ImmediateParser::parse(navigator)
            }
            TokenKind::Punctuation if token.value == "[" => {
                IndirectParser::parse(navigator)
            }
            TokenKind::Identifier if OperandValidator::is_register(&token.value) => {
                RegisterParser::parse(navigator)
            }
            TokenKind::Punctuation if token.value == "+" || token.value == "-" => {
                RelativeParser::parse_with_sign(navigator)
            }
            TokenKind::Number => {
                let number_token = navigator.peek().unwrap();
                if number_token.value.starts_with('-') {
                    RelativeParser::parse_negative_number(navigator)
                } else {
                    DirectParser::parse_number_or_indexed(navigator)
                }
            }
            _ => {
                DirectParser::parse_identifier_or_indexed(navigator)
            }
        }
    }
}
