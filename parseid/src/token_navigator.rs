//! Token navigation utilities

use lexariel::{Token, TokenKind};
use crate::error::ParserError;

/// Helper struct for navigating through tokens
pub(crate) struct TokenNavigator {
    tokens: Vec<Token>,
    position: usize,
}

impl TokenNavigator {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    /// returns the current token without advancing
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// returns the token at the given offset without advancing
    pub fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    /// advances to the next token and returns it
    pub fn advance(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    /// checks if the current token matches the expected kind
    pub fn check(&self, kind: TokenKind) -> bool {
        if let Some(token) = self.peek() {
            token.kind == kind
        } else {
            false
        }
    }

    /// consumes a token of the expected kind or returns an error
    pub fn consume(&mut self, kind: TokenKind, expected: &str) -> Result<Token, ParserError> {
        if let Some(token) = self.peek() {
            if token.kind == kind {
                Ok(self.advance().unwrap().clone())
            } else {
                Err(ParserError::UnexpectedToken {
                    line: token.line,
                    column: token.column,
                    expected: expected.to_string(),
                    found: format!("{}", token.kind),
                })
            }
        } else {
            Err(ParserError::UnexpectedEof {
                expected: expected.to_string(),
            })
        }
    }

    /// consumes a value token (number or identifier)
    pub fn consume_value(&mut self, expected: &str) -> Result<Token, ParserError> {
        let token = self.peek().ok_or(ParserError::UnexpectedEof {
            expected: expected.to_string(),
        })?;

        match token.kind {
            TokenKind::Number | TokenKind::Identifier => Ok(self.advance().unwrap().clone()),
            _ => Err(ParserError::UnexpectedToken {
                line: token.line,
                column: token.column,
                expected: expected.to_string(),
                found: format!("{}", token.kind),
            }),
        }
    }
}
