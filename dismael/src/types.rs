//! types for the dismael disassembler

#[derive(Debug, Clone, PartialEq)]
pub struct DisassembledInstruction {
    pub address: u16,
    pub raw_value: u16,
    pub opcode: u8,
    pub argument: u16,
    pub mnemonic: String,
    pub operand: Option<String>,
    pub is_data: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddressingMode {
    None,
    Direct,
    Immediate,
    Indirect,
    Register,
    Relative,
}
