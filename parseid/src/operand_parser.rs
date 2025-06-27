//! Operand parsing functionality

use lexariel::TokenKind;
use crate::ast::{Operand, AddressingMode};
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

/// Parses operands with different addressing modes
pub(crate) struct OperandParser;

impl OperandParser {
    /// parses an operand with addressing mode
    pub fn parse_operand(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        // do we have a token for operand?
        if navigator.peek().is_none() || navigator.peek().unwrap().kind == TokenKind::Eof {
            return Err(ParserError::UnexpectedEof {
                expected: "operand".to_string(),
            });
        }

        // check if the next token is something that would start a new statement
        let token = navigator.peek().unwrap();
        match token.kind {
            TokenKind::LabelDef | TokenKind::Keyword | TokenKind::Directive => {
                return Err(ParserError::UnexpectedEof {
                    expected: "operand".to_string(),
                });
            }
            _ => {}
        }

        let token = navigator.peek().unwrap();

        match &token.kind {
            TokenKind::Punctuation if token.value == "#" => {
                // immediate addressing: #value
                navigator.advance(); // consume #
                let value_token = navigator.consume_value("immediate value")?;
                Ok(Operand {
                    addressing_mode: AddressingMode::Immediate,
                    value: value_token.value,
                })
            }
            TokenKind::Punctuation if token.value == "[" => {
                // indirect or multiple indirect addressing
                Self::parse_indirect_operand(navigator)
            }
            TokenKind::Identifier if token.value.to_uppercase().starts_with('R') => {
                // register addressing
                Self::parse_register_operand(navigator)
            }
            TokenKind::Punctuation if token.value == "+" || token.value == "-" => {
                // relative addressing
                let sign_token = navigator.advance().unwrap().clone();
                let offset_token = navigator.consume_value("offset value")?;
                Ok(Operand {
                    addressing_mode: AddressingMode::Relative,
                    value: format!("{}{}", sign_token.value, offset_token.value),
                })
            }
            TokenKind::Number => {
                // could be negative number (relative) or positive number (direct)
                let number_token = navigator.advance().unwrap().clone();
                if number_token.value.starts_with('-') {
                    Ok(Operand {
                        addressing_mode: AddressingMode::Relative,
                        value: number_token.value,
                    })
                } else {
                    // indexed addressing: number[index]
                    if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
                        navigator.advance(); // consume [
                        let index_token = navigator.consume_value("index")?;
                        let index_value = index_token.value.clone();
                        navigator.consume(TokenKind::Punctuation, "]")?;
                        
                        Ok(Operand {
                            addressing_mode: AddressingMode::Indexed {
                                address: number_token.value.clone(),
                                index: index_value.clone(),
                            },
                            value: format!("{}[{}]", number_token.value, index_value),
                        })
                    } else {
                        // direct addressing
                        Ok(Operand {
                            addressing_mode: AddressingMode::Direct,
                            value: number_token.value,
                        })
                    }
                }
            }
            _ => {
                // direct addressing or indexed addressing (identifiers)
                Self::parse_direct_or_indexed_operand(navigator)
            }
        }
    }

    fn parse_indirect_operand(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        navigator.consume(TokenKind::Punctuation, "[")?;

        // multiple indirect [[...]]
        if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
            navigator.advance(); // consume second [
            let value_token = navigator.consume_value("address")?;
            let value = value_token.value.clone();
            navigator.consume(TokenKind::Punctuation, "]")?; // consume first ]
            navigator.consume(TokenKind::Punctuation, "]")?; // consume second ]
            
            Ok(Operand {
                addressing_mode: AddressingMode::MultipleIndirect,
                value,
            })
        } else {
            // register indirect [R0] or regular indirect [address]
            let value_token = navigator.peek().ok_or(ParserError::UnexpectedEof {
                expected: "address or register".to_string(),
            })?;

            if value_token.kind == TokenKind::Identifier && value_token.value.to_uppercase().starts_with('R') {
                // indirect [R0]
                let reg_token = navigator.advance().unwrap().clone();
                navigator.consume(TokenKind::Punctuation, "]")?;
                
                Ok(Operand {
                    addressing_mode: AddressingMode::RegisterIndirect,
                    value: reg_token.value,
                })
            } else {
                // indirect [address]
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

    fn parse_register_operand(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let reg_token = navigator.consume(TokenKind::Identifier, "register")?;
        let reg_name = reg_token.value.clone();
        
        // base register addressing R0[offset]
        if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
            navigator.advance(); // consume [
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
        } else {
            // simple register addressing
            Ok(Operand {
                addressing_mode: AddressingMode::Register,
                value: reg_name,
            })
        }
    }

    fn parse_direct_or_indexed_operand(navigator: &mut TokenNavigator) -> Result<Operand, ParserError> {
        let addr_token = navigator.consume_value("address")?;
        let addr_value = addr_token.value.clone();
        
        // indexed addressing address[index]
        if navigator.check(TokenKind::Punctuation) && navigator.peek().unwrap().value == "[" {
            navigator.advance(); // consume [
            let index_token = navigator.consume_value("index")?;
            let index_value = index_token.value.clone();
            navigator.consume(TokenKind::Punctuation, "]")?;
            
            Ok(Operand {
                addressing_mode: AddressingMode::Indexed {
                    address: addr_value.clone(),
                    index: index_value.clone(),
                },
                value: format!("{}[{}]", addr_value, index_value),
            })
        } else {
            // direct addressing
            Ok(Operand {
                addressing_mode: AddressingMode::Direct,
                value: addr_value,
            })
        }
    }
}
