use lexariel::TokenKind;
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

pub(crate) struct OperandValidator;

impl OperandValidator {
    pub fn validate_has_operand(navigator: &mut TokenNavigator) -> Result<(), ParserError> {
        // token for operand?
        if navigator.peek().is_none() || navigator.peek().unwrap().kind == TokenKind::Eof {
            return Err(ParserError::UnexpectedEof {
                expected: "operand".to_string(),
            });
        }

        // next token is something that would start a new statement?
        let token = navigator.peek().unwrap();
        match token.kind {
            TokenKind::LabelDef | TokenKind::Keyword | TokenKind::Directive => {
                return Err(ParserError::UnexpectedEof {
                    expected: "operand".to_string(),
                });
            }
            _ => {}
        }

        Ok(())
    }

    pub fn is_register(value: &str) -> bool {
        let upper = value.to_uppercase();
        if !upper.starts_with('R') || upper.len() < 2 {
            return false;
        }
        
        let rest = &upper[1..];
        rest.chars().all(|c| c.is_ascii_digit())
    }
}
