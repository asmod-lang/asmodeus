//! core emulator implementation

use std::collections::HashSet;

mod memory;
mod stack;
mod interrupts;
mod io;
mod addressing;
mod execution;

/// - operating memory (PaO): 2048 words, 16-bit each
/// - registers: AK (accumulator), L (instruction counter), AD (address), KOD (opcode), WS (stack pointer)
/// - execution state and control flags
/// - interrupt handling system

#[derive(Debug, Clone)]
pub struct MachineW {
    /// operating memory - 2048 words of 16 bits each
    pub memory: Vec<u16>,
    
    /// AK - accumulator register (16-bit)
    pub ak: u16,
    
    /// L - instruction counter (11-bit, range 0-2047)
    pub l: u16,
    
    /// AD - address register (11-bit)
    pub ad: u16,
    
    /// KOD - opcode register (5-bit)
    pub kod: u8,
    
    /// WS - stack pointer (11-bit, grows downward, initialized to 2047)
    pub ws: u16,
    
    /// execution control flag
    pub is_running: bool,
    
    /// interrupt control flags
    pub interrupts_enabled: bool,
    pub interrupt_mask: u16,
    
    /// pending interrupt vector (if any)
    pub pending_interrupt: Option<u16>,
    
    /// general purpose registers (for extended addressing modes)
    pub registers: [u16; 8], // R0-R7
    
    /// I/O simulation buffers
    pub input_buffer: Vec<u16>,
    pub output_buffer: Vec<u16>,

    pub breakpoints: HashSet<u16>,

    pub interactive_mode: bool,
}

impl Default for MachineW {
    fn default() -> Self {
        Self::new()
    }
}

impl MachineW {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 2048],
            ak: 0,
            l: 0,
            ad: 0,
            kod: 0,
            ws: 2047, // stack pointer initialized to top of memory (grows downward)
            is_running: false,
            interrupts_enabled: true,
            interrupt_mask: 0,
            pending_interrupt: None,
            registers: [0; 8],
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
            breakpoints: HashSet::new(),
            interactive_mode: false,
        }
    }

    pub fn set_interactive_mode(&mut self, enabled: bool) {
        self.interactive_mode = enabled;
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.ak = 0;
        self.l = 0;
        self.ad = 0;
        self.kod = 0;
        self.ws = 2047;
        self.is_running = false;
        self.interrupts_enabled = true;
        self.interrupt_mask = 0;
        self.pending_interrupt = None;
        self.registers.fill(0);
        self.input_buffer.clear();
        self.output_buffer.clear();
        self.breakpoints.clear();
    }
}
