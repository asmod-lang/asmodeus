//! R0, R0[offset]

use lexariel::TokenKind;
use crate::ast::{Operand, AddressingMode};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

pub(crate) struct RegisterParser;

impl RegisterParser {
    pub fn parse(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let reg_token = navigator.consume(TokenKind::Identifier, "register")?;
        let reg_name = reg_token.value.clone();
        
        // R0[offset]
        if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
            Self::parse_base_register(navigator, reg_name)
        } else {
            // simple register addressing
            Ok(Operand {
                addressing_mode: AddressingMode::Register,
                value: reg_name,
            })
        }
    }

    fn parse_base_register(navigator: &mut TokenNavigator, reg_name: String) -> Result<Operand, ParserError> {
        navigator.advance(); // [
        let offset_token = navigator.consume_value("offset")?;
        let offset_value = offset_token.value.clone();
        navigator.consume(TokenKind::Punctuation, "]")?;
        
        Ok(Operand {
            addressing_mode: AddressingMode::BaseRegister {
                base: reg_name.clone(),
                offset: offset_value.clone(),
            },
            value: format!("{}[{}]", reg_name, offset_value),
        })
    }
}
