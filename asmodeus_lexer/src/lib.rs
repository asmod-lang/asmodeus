//! Lexer for Asmodeus assembly language
//! Converts assembly code into a stream of tokens for Machine W

use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LexerError {
    #[error("Unknown token at line {line}, column {column}: '{token}'")]
    UnknownToken { line: usize, column: usize, token: String },
    #[error("Invalid number format at line {line}, column {column}: '{value}'")]
    InvalidNumberFormat { line: usize, column: usize, value: String },
    #[error("Unterminated string at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },
    #[error("Invalid character at line {line}, column {column}: '{character}'")]
    InvalidCharacter { line: usize, column: usize, character: char },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// assembly-like instructions (DOD, ODE, ŁAD, POB, SOB, SOM, STP, DNS, PZS, SDP, CZM, MSK, PWR, WEJSCIE, WYJSCIE)
    Keyword,
    /// assembly-like directives (RST, RPA, MAKRO, KONM, NAZWA_LOKALNA)
    Directive,
    /// variable names, labels, macro names
    Identifier,
    /// integer numbers (decimal, hexadecimal, binary)
    Number,
    /// label definitions (identifier followed by colon)
    LabelDef,
    /// punctuation marks (,, [, ], #, etc.)
    Punctuation,
    /// end of file marker
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Keyword => write!(f, "Keyword"),
            TokenKind::Directive => write!(f, "Directive"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Number => write!(f, "Number"),
            TokenKind::LabelDef => write!(f, "LabelDef"),
            TokenKind::Punctuation => write!(f, "Punctuation"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// type of the token
    pub kind: TokenKind,
    /// string value of the token
    pub value: String,
    /// line number (1-based)
    pub line: usize,
    /// column number (1-based)
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, value: String, line: usize, column: usize) -> Self {
        Self { kind, value, line, column }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}) at {}:{}", self.kind, self.value, self.line, self.column)
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// returns the current character without advancing
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// returns the character at the given offset without advancing
    fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    /// advances to the next character and returns it
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.input.get(self.position) {
            self.position += 1;
            if *ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(*ch)
        } else {
            None
        }
    }

    /// skips whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// skips comment lines (starting with ; or //)
    fn skip_comment(&mut self) {
        if let Some(ch) = self.peek() {
            if ch == ';' {
                // skip until end of line
                while let Some(ch) = self.advance() {
                    if ch == '\n' {
                        break;
                    }
                }
            } else if ch == '/' && self.peek_ahead(1) == Some('/') {
                // skip // comment
                self.advance(); // skip first /
                self.advance(); // skip second /
                while let Some(ch) = self.advance() {
                    if ch == '\n' {
                        break;
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        let start_line = self.line;
        let start_column = self.column;

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    fn read_number(&mut self) -> Result<String, LexerError> {
        let mut number = String::new();
        let start_line = self.line;
        let start_column = self.column;

        // negative numbers
        if self.peek() == Some('-') {
            number.push('-');
            self.advance();
        }

        // check for hex (0x) or binary (0b) prefix
        if self.peek() == Some('0') {
            if let Some(next) = self.peek_ahead(1) {
                if next == 'x' || next == 'X' {
                    // hex - preserve original case
                    number.push('0');
                    self.advance(); // skip '0'
                    number.push(next); // preserve original 'x' or 'X'
                    self.advance(); // skip 'x'/'X'
                    
                    let mut has_digits = false;
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_hexdigit() {
                            number.push(ch);
                            self.advance();
                            has_digits = true;
                        } else {
                            break;
                        }
                    }
                    
                    if !has_digits {
                        return Err(LexerError::InvalidNumberFormat {
                            line: start_line,
                            column: start_column,
                            value: number,
                        });
                    }
                } else if next == 'b' || next == 'B' {
                    // binary - preserve original case
                    number.push('0');
                    self.advance(); // skip '0'
                    number.push(next); // preserve original 'b' or 'B'
                    self.advance(); // skip 'b'/'B'
                    
                    let mut has_digits = false;
                    while let Some(ch) = self.peek() {
                        if ch == '0' || ch == '1' {
                            number.push(ch);
                            self.advance();
                            has_digits = true;
                        } else {
                            break;
                        }
                    }
                    
                    if !has_digits {
                        return Err(LexerError::InvalidNumberFormat {
                            line: start_line,
                            column: start_column,
                            value: number,
                        });
                    }
                } else {
                    // regular decimal starting with 0
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_digit() {
                            number.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            } else {
                // just '0'
                number.push('0');
                self.advance();
            }
        } else {
            // regular decimal number
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    number.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        if number.is_empty() || number == "-" {
            return Err(LexerError::InvalidNumberFormat {
                line: start_line,
                column: start_column,
                value: number,
            });
        }

        Ok(number)
    }

    fn is_keyword(word: &str) -> bool {
        matches!(word.to_uppercase().as_str(),
            "DOD" | "ODE" | "ŁAD" | "LAD" | "POB" | "SOB" | "SOM" | "SOZ" | "STP" | 
            "DNS" | "PZS" | "SDP" | "CZM" | "MSK" | "PWR" | "WEJSCIE" | "WYJSCIE"
        )
    }

    fn is_directive(word: &str) -> bool {
        matches!(word.to_uppercase().as_str(),
            "RST" | "RPA" | "MAKRO" | "KONM" | "NAZWA_LOKALNA"
        )
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        loop {
            self.skip_whitespace();
            
            // check for comments
            if self.peek() == Some(';') || 
               (self.peek() == Some('/') && self.peek_ahead(1) == Some('/')) {
                self.skip_comment();
                continue;
            }

            let start_line = self.line;
            let start_column = self.column;

            match self.peek() {
                None => {
                    return Ok(Token::new(TokenKind::Eof, String::new(), start_line, start_column));
                }
                
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    
                    // if next character is colon (label definition)
                    if self.peek() == Some(':') {
                        self.advance(); // consume the colon
                        return Ok(Token::new(
                            TokenKind::LabelDef,
                            identifier,
                            start_line,
                            start_column,
                        ));
                    }
                    
                    // determine token type
                    let kind = if Self::is_keyword(&identifier) {
                        TokenKind::Keyword
                    } else if Self::is_directive(&identifier) {
                        TokenKind::Directive
                    } else {
                        TokenKind::Identifier
                    };
                    
                    return Ok(Token::new(kind, identifier, start_line, start_column));
                }
                
                Some(ch) if ch.is_ascii_digit() || ch == '-' => {
                    // negative number or just a minus sign
                    if ch == '-' {
                        if let Some(next) = self.peek_ahead(1) {
                            if !next.is_ascii_digit() {
                                // just a minus punctuation
                                self.advance();
                                return Ok(Token::new(
                                    TokenKind::Punctuation,
                                    ch.to_string(),
                                    start_line,
                                    start_column,
                                ));
                            }
                        } else {
                            // end of input, just a minus
                            self.advance();
                            return Ok(Token::new(
                                TokenKind::Punctuation,
                                ch.to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                    }
                    
                    let number = self.read_number()?;
                    return Ok(Token::new(TokenKind::Number, number, start_line, start_column));
                }
                
                Some(ch) if "[]{}(),.#+-*/=<>!&|^~".contains(ch) => {
                    self.advance();
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let tokens = tokenize("").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn test_whitespace_only() {
        let tokens = tokenize("   \t\n\r  ").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn test_simple_keyword() {
        let tokens = tokenize("DOD").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[0].value, "DOD");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
}
