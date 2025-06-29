use super::AddressingMode;

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub addressing_mode: AddressingMode,
    pub value: String,
}
