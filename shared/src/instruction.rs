/// [5 opcode bits][3 address bits][8 argument bits]
pub fn encode_instruction(opcode: u8, addressing_mode: u8, argument: u16) -> u16 {
    let opcode_bits = (opcode as u16 & 0b11111) << 11;
    let mode_bits = (addressing_mode as u16 & 0b111) << 8;
    let arg_bits = argument & 0xFF;
    
    opcode_bits | mode_bits | arg_bits
}

pub fn decode_instruction(instruction: u16) -> (u8, u8, u16) {
    let opcode = extract_opcode(instruction);
    let addressing_mode = extract_addressing_mode(instruction);
    let argument = extract_argument(instruction);
    
    (opcode, addressing_mode, argument)
}

/// 15-11 bits
pub fn extract_opcode(instruction: u16) -> u8 {
    ((instruction >> 11) & 0b11111) as u8
}

/// 10-8 bits
pub fn extract_addressing_mode(instruction: u16) -> u8 {
    ((instruction >> 8) & 0b111) as u8
}

/// bity 7-0 bits
pub fn extract_argument(instruction: u16) -> u16 {
    instruction & 0xFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_encoding() {
        let instruction = encode_instruction(0b00001, 0b000, 100);
        assert_eq!(instruction, (0b00001u16 << 11) | (0b000u16 << 8) | 100);
    }

    #[test]
    fn test_instruction_decoding() {
        let instruction = (0b00001u16 << 11) | (0b000u16 << 8) | 100;
        let (opcode, mode, arg) = decode_instruction(instruction);
        assert_eq!(opcode, 0b00001);
        assert_eq!(mode, 0b000);
        assert_eq!(arg, 100);
    }
}
