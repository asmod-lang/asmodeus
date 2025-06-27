//! types and data structures for emulator

#[derive(Debug, Clone, PartialEq)]
pub enum AddressingMode {
    /// no operand was specified
    None,
    /// immediate addressing (#value)
    Immediate(u16),
    /// direct addressing (address)
    Direct(u16),
    /// indirect addressing ([address])
    Indirect(u16),
    /// register addressing (Rn)
    Register(u8),
    /// register indirect ([Rn])
    RegisterIndirect(u8),
    /// indexed addressing (base[index])
    Indexed(u16, u16),
    /// relative addressing (+/-offset)
    Relative(i16),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MachineWState {
    pub ak: u16,
    pub l: u16,
    pub ad: u16,
    pub kod: u8,
    pub ws: u16,
    pub is_running: bool,
    pub interrupts_enabled: bool,
    pub interrupt_mask: u16,
    pub registers: [u16; 8],
}
