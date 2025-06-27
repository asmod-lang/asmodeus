//! token types and definitions

use std::fmt;

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
