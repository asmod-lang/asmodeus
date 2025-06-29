pub mod addressing;
pub mod instruction;
pub mod opcodes;

pub use addressing::{AddressingModeBits, addressing_mode_bits};
pub use instruction::{encode_instruction, decode_instruction, extract_opcode, extract_addressing_mode, extract_argument};
pub use opcodes::Opcode;
