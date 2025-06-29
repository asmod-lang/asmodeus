//! value

use crate::ast::{Operand, AddressingMode};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

pub(crate) struct ImmediateParser;

impl ImmediateParser {
    pub fn parse(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        navigator.advance(); // #
        let value_token = navigator.consume_value("immediate value")?;
        Ok(Operand {
            addressing_mode: AddressingMode::Immediate,
            value: value_token.value,
        })
    }
}
