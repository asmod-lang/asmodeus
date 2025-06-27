//! directive parser

use lexariel::TokenKind;
use crate::ast::Directive;
use crate::error::ParserError;
use crate::token_navigator::TokenNavigator;

/// Parses directives
pub(crate) struct DirectiveParser;

impl DirectiveParser {
    pub fn parse_directive(navigator: &mut TokenNavigator) -> Result<Directive, ParserError> {
        let token = navigator.consume(TokenKind::Directive, "directive")?;
        let name = token.value;
        let line = token.line;
        let column = token.column;

        let mut arguments = Vec::new();

        while let Some(arg_token) = navigator.peek() {
            match arg_token.kind {
                TokenKind::Number | TokenKind::Identifier => {
                    arguments.push(navigator.advance().unwrap().value.clone());
                }
                TokenKind::Eof | TokenKind::LabelDef | TokenKind::Keyword | TokenKind::Directive => {
                    break;
                }
                _ => {
                    navigator.advance(); // skip other tokens (...punctuation)
                }
            }
        }

        Ok(Directive {
            name,
            arguments,
            line,
            column,
        })
    }
}
