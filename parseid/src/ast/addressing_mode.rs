use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AddressingMode {
    /// #value
    Immediate,
    /// address
    Direct,
    /// [address]
    Indirect,
    /// [[address]]
    MultipleIndirect,
    /// R0, R1, etc.
    Register,
    /// [R0]
    RegisterIndirect,
    /// base[offset]
    BaseRegister { base: String, offset: String },
    /// +offset or -offset
    Relative,
    /// address[index]
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
