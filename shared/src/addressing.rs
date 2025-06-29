#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressingModeBits;

impl AddressingModeBits {
    pub const DIRECT: u8 = 0b000;
    pub const IMMEDIATE: u8 = 0b001;
    pub const INDIRECT: u8 = 0b010;
    pub const MULTIPLE_INDIRECT: u8 = 0b011;
    pub const REGISTER: u8 = 0b100;
    pub const REGISTER_INDIRECT: u8 = 0b101;
    pub const BASE_REGISTER: u8 = 0b110;
    pub const RELATIVE: u8 = 0b111;
}

pub mod addressing_mode_bits {
    pub const DIRECT: u8 = 0b000;
    pub const IMMEDIATE: u8 = 0b001;
    pub const INDIRECT: u8 = 0b010;
    pub const MULTIPLE_INDIRECT: u8 = 0b011;
    pub const REGISTER: u8 = 0b100;
    pub const REGISTER_INDIRECT: u8 = 0b101;
    pub const BASE_REGISTER: u8 = 0b110;
    pub const RELATIVE: u8 = 0b111;
}

pub fn is_valid_addressing_mode(mode: u8) -> bool {
    mode <= 7
}

pub fn is_valid_register(register: u8) -> bool {
    register <= 7
}
