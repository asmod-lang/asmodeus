//! [address], [[address]], [R0]

use lexariel::TokenKind;
use crate::ast::{Operand, AddressingMode};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

pub(crate) struct IndirectParser;

impl IndirectParser {
    pub fn parse(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        navigator.consume(TokenKind::Punctuation, "[")?;

        // multiple indirect [[...]]
        if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
            Self::parse_multiple_indirect(navigator)
        } else {
            Self::parse_single_indirect(navigator)
        }
    }

    fn parse_multiple_indirect(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        navigator.advance(); // second [
        let value_token = navigator.consume_value("address")?;
        let value = value_token.value.clone();
        navigator.consume(TokenKind::Punctuation, "]")?; // first ]
        navigator.consume(TokenKind::Punctuation, "]")?; // second ]
        
        Ok(Operand {
            addressing_mode: AddressingMode::MultipleIndirect,
            value,
        })
    }

    fn parse_single_indirect(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let value_token = navigator.peek().ok_or(ParserError::UnexpectedEof {
            expected: "address or register".to_string(),
        })?;

        if value_token.kind == TokenKind::Identifier && value_token.value.to_uppercase().starts_with('R') {
            // register indirect [R0]
            let reg_token = navigator.advance().unwrap().clone();
            navigator.consume(TokenKind::Punctuation, "]")?;
            
            Ok(Operand {
                addressing_mode: AddressingMode::RegisterIndirect,
                value: reg_token.value,
            })
        } else {
            // regular indirect [address]
            let addr_token = navigator.consume_value("address")?;
            let value = addr_token.value.clone();
            navigator.consume(TokenKind::Punctuation, "]")?;
            
            Ok(Operand {
                addressing_mode: AddressingMode::Indirect,
                value,
            })
        }
    }
}
