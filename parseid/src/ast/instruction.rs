use super::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub operand: Option<Operand>,
    pub line: usize,
    pub column: usize,
}
