//! symbol table management for labels & variables

use crate::error::AssemblerError;
use crate::types::{Symbol, SymbolType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, address: u16, symbol_type: SymbolType) -> Result<(), AssemblerError> {
        if self.symbols.contains_key(&name) {
            return Err(AssemblerError::DuplicateSymbol { 
                symbol: name, 
                line: 0 // will be filled by caller
            });
        }
        self.symbols.insert(name, Symbol { address, symbol_type });
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn get_address(&self, name: &str) -> Option<u16> {
        self.symbols.get(name).map(|s| s.address)
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
