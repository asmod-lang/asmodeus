//! +offset, -offset

use crate::ast::{Operand, AddressingMode};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

pub(crate) struct RelativeParser;

impl RelativeParser {
    pub fn parse_with_sign(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let sign_token = navigator.advance().unwrap().clone();
        let offset_token = navigator.consume_value("offset value")?;
        Ok(Operand {
            addressing_mode: AddressingMode::Relative,
            value: format!("{}{}", sign_token.value, offset_token.value),
        })
    }

    pub fn parse_negative_number(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let number_token = navigator.advance().unwrap().clone();
        Ok(Operand {
            addressing_mode: AddressingMode::Relative,
            value: number_token.value,
        })
    }
}
