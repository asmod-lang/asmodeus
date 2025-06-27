//! types for hephasm assembler

use parseid::ast::ProgramElement;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub address: u16,
    pub symbol_type: SymbolType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Label,
    Variable,
}

#[derive(Debug, Clone)]
pub struct ExpandedMacro {
    pub parameters: Vec<String>,
    pub body: Vec<ProgramElement>,
}
