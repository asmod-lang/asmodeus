#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    DOD = 0b00001,    // ADD 
    ODE = 0b00010,    // SUBSTRACT 
    LAD = 0b00011,    // Load Accumulator 
    POB = 0b00100,    // Get From Memory
    SOB = 0b00101,    // Unconditionally Jump 
    SOM = 0b00110,    // Jump if Less Than Zero 
    STP = 0b00111,    // Stop
    DNS = 0b01000,    // Turn Off Interrupts 
    PZS = 0b01001,    // Pop From Stack 
    SDP = 0b01010,    // Push To Stack 
    CZM = 0b01011,    // Clear Interrupt Mask 
    MSK = 0b01100,    // Set Interrupt Mask 
    PWR = 0b01101,    // Return From Interrupt 
    WEJSCIE = 0b01110, // Input Operation 
    WYJSCIE = 0b01111, // Output Operation 
    SOZ = 0b10000,    // Jump if Zero 
}

impl Opcode {
    /// u8 -> Opcode
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00001 => Some(Self::DOD),
            0b00010 => Some(Self::ODE),
            0b00011 => Some(Self::LAD),
            0b00100 => Some(Self::POB),
            0b00101 => Some(Self::SOB),
            0b00110 => Some(Self::SOM),
            0b00111 => Some(Self::STP),
            0b01000 => Some(Self::DNS),
            0b01001 => Some(Self::PZS),
            0b01010 => Some(Self::SDP),
            0b01011 => Some(Self::CZM),
            0b01100 => Some(Self::MSK),
            0b01101 => Some(Self::PWR),
            0b01110 => Some(Self::WEJSCIE),
            0b01111 => Some(Self::WYJSCIE),
            0b10000 => Some(Self::SOZ),
            _ => None,
        }
    }

    pub fn requires_operand(self) -> bool {
        match self {
            Self::STP | Self::DNS | Self::PZS | Self::SDP | Self::CZM | Self::PWR => false,
            _ => true,
        }
    }
}
