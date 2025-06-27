//! Parser for Asmodeus assembly language (Parseid)
//! Converts tokens into Abstract Syntax Tree (AST)

pub mod ast;
mod error;
mod token_navigator;
mod operand_parser;
mod instruction_parser;
mod directive_parser;
mod macro_parser;
mod parser;

pub use error::ParserError;
pub use parser::Parser;
pub use ast::*;

use lexariel::Token;

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParserError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn parse_source(source: &str) -> Result<Program, ParserError> {
    let tokens = lexariel::tokenize(source)?;
    parse(tokens)
}
