//! (address, address[index])

use lexariel::TokenKind;
use crate::ast::{Operand, AddressingMode};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

pub(crate) struct DirectParser;

impl DirectParser {
    pub fn parse_number_or_indexed(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let number_token = navigator.advance().unwrap().clone();
        
        // number[index]
        if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
            Self::parse_indexed(navigator, number_token.value)
        } else {
            // direct addressing
            Ok(Operand {
                addressing_mode: AddressingMode::Direct,
                value: number_token.value,
            })
        }
    }

    pub fn parse_identifier_or_indexed(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let addr_token = navigator.consume_value("address")?;
        let addr_value = addr_token.value.clone();
        
        // address[index]
        if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
            Self::parse_indexed(navigator, addr_value)
        } else {
            // direct addressing
            Ok(Operand {
                addressing_mode: AddressingMode::Direct,
                value: addr_value,
            })
        }
    }

    fn parse_indexed(navigator: &mut TokenNavigator, address: String) -> Result<Operand, ParserError> {
        navigator.advance(); // [
        let index_token = navigator.consume_value("index")?;
        let index_value = index_token.value.clone();
        navigator.consume(TokenKind::Punctuation, "]")?;
        
        Ok(Operand {
            addressing_mode: AddressingMode::Indexed {
                address: address.clone(),
                index: index_value.clone(),
            },
            value: format!("{}[{}]", address, index_value),
        })
    }
}
