//! core lexer implementation

use crate::error::LexerError;
use crate::token::{Token, TokenKind};
use crate::keywords::{is_keyword, is_directive};
use crate::position::InputReader;
use crate::parsers::{read_identifier, read_number, skip_whitespace, skip_comment};

pub struct Lexer {
    reader: InputReader,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            reader: InputReader::new(input),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        loop {
            skip_whitespace(&mut self.reader);
            
            // check for comments
            if self.reader.peek() == Some(';') || 
               (self.reader.peek() == Some('/') && self.reader.peek_ahead(1) == Some('/')) {
                skip_comment(&mut self.reader);
                continue;
            }

            let start_line = self.reader.line();
            let start_column = self.reader.column();

            match self.reader.peek() {
                None => {
                    return Ok(Token::new(TokenKind::Eof, String::new(), start_line, start_column));
                }
                
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    let identifier = read_identifier(&mut self.reader);
                    
                    // if next character is colon (label definition)
                    if self.reader.peek() == Some(':') {
                        self.reader.advance(); // consume the colon
                        return Ok(Token::new(
                            TokenKind::LabelDef,
                            identifier,
                            start_line,
                            start_column,
                        ));
                    }
                    
                    // determine token type
                    let kind = if is_keyword(&identifier) {
                        TokenKind::Keyword
                    } else if is_directive(&identifier) {
                        TokenKind::Directive
                    } else {
                        TokenKind::Identifier
                    };
                    
                    return Ok(Token::new(kind, identifier, start_line, start_column));
                }
                
                Some(ch) if ch.is_ascii_digit() || ch == '-' => {
                    // negative number or just a minus sign
                    if ch == '-' {
                        if let Some(next) = self.reader.peek_ahead(1) {
                            if !next.is_ascii_digit() {
                                // just a minus punctuation
                                self.reader.advance();
                                return Ok(Token::new(
                                    TokenKind::Punctuation,
                                    ch.to_string(),
                                    start_line,
                                    start_column,
                                ));
                            }
                        } else {
                            // end of input, just a minus
                            self.reader.advance();
                            return Ok(Token::new(
                                TokenKind::Punctuation,
                                ch.to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                    }
                    
                    let number = read_number(&mut self.reader)?;
                    return Ok(Token::new(TokenKind::Number, number, start_line, start_column));
                }
                
                Some(ch) if "[]{}(),.#+-*/=<>!&|^~".contains(ch) => {
                    self.reader.advance();
                    return Ok(Token::new(
                        TokenKind::Punctuation,
                        ch.to_string(),
                        start_line,
                        start_column,
                    ));
                }
                
                Some(ch) => {
                    return Err(LexerError::InvalidCharacter {
                        line: start_line,
                        column: start_column,
                        character: ch,
                    });
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}
