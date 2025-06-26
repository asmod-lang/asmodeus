//! Abstract Syntax Tree definitions for Asmodeus assembly language

use std::fmt;

/// main program node containing all top-level elements
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub elements: Vec<ProgramElement>,
}

/// top-level program elements
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramElement {
    Instruction(Instruction),
    LabelDefinition(LabelDefinition),
    Directive(Directive),
    MacroDefinition(MacroDefinition),
    MacroCall(MacroCall),
}

/// assembly instruction with opcode and operand
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub operand: Option<Operand>,
    pub line: usize,
    pub column: usize,
}

/// operand with addressing mode
#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub addressing_mode: AddressingMode,
    pub value: String,
}

/// addressing modes for Machine W (based on SuperW architecture)
/// TODO: probably need to extend this with more
#[derive(Debug, Clone, PartialEq)]
pub enum AddressingMode {
    /// immediate addressing: #value
    Immediate,
    /// direct addressing: address
    Direct,
    /// indirect addressing: [address]
    Indirect,
    /// multiple indirect addressing: [[address]]
    MultipleIndirect,
    /// register addressing: R0, R1, etc.
    Register,
    /// register indirect addressing: [R0]
    RegisterIndirect,
    /// base register addressing: base[offset]
    BaseRegister { base: String, offset: String },
    /// relative addressing: +offset or -offset
    Relative,
    /// indexed addressing: address[index]
    Indexed { address: String, index: String },
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressingMode::Immediate => write!(f, "Immediate"),
            AddressingMode::Direct => write!(f, "Direct"),
            AddressingMode::Indirect => write!(f, "Indirect"),
            AddressingMode::MultipleIndirect => write!(f, "MultipleIndirect"),
            AddressingMode::Register => write!(f, "Register"),
            AddressingMode::RegisterIndirect => write!(f, "RegisterIndirect"),
            AddressingMode::BaseRegister { base, offset } => write!(f, "BaseRegister({}, {})", base, offset),
            AddressingMode::Relative => write!(f, "Relative"),
            AddressingMode::Indexed { address, index } => write!(f, "Indexed({}, {})", address, index),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelDefinition {
    pub name: String,
    pub line: usize,
    pub column: usize,
}

/// assembler directive (..., RST, RPA, ...)
#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub name: String,
    pub arguments: Vec<String>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDefinition {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<ProgramElement>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroCall {
    pub name: String,
    pub arguments: Vec<String>,
    pub line: usize,
    pub column: usize,
}

impl Program {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn add_element(&mut self, element: ProgramElement) {
        self.elements.push(element);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
