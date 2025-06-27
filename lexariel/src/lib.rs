//! Lexer (Lexariel) for Asmodeus assembly language
//! Converts assembly code into a stream of tokens for Machine W (asmachina)

mod error;
mod token;
mod keywords;
mod position;
mod parsers;
mod lexer;

pub use error::LexerError;
pub use token::{Token, TokenKind};
pub use lexer::Lexer;

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
