//! core emulator implementation

use crate::error::MachineError;
use std::collections::HashSet;

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

    pub fn read_memory(&self, address: u16) -> Result<u16, MachineError> {
        let addr = address & 0b0000011111111111; // limit to 11 bits (0-2047)
        if addr >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address: addr });
        }
        Ok(self.memory[addr as usize])
    }

    pub fn write_memory(&mut self, address: u16, value: u16) -> Result<(), MachineError> {
        let addr = address & 0b0000011111111111; // limit to 11 bits (0-2047)
        if addr >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address: addr });
        }
        self.memory[addr as usize] = value;
        Ok(())
    }

    /// loads program into memory (starting at address 0!)
    pub fn load_program(&mut self, program: &[u16]) -> Result<(), MachineError> {
        if program.len() > 2048 {
            return Err(MachineError::AddressOutOfBounds { 
                address: program.len() as u16 
            });
        }
        
        for (i, &instruction) in program.iter().enumerate() {
            self.memory[i] = instruction;
        }
        Ok(())
    }

    fn fetch_and_decode(&mut self) -> Result<(), MachineError> {
        // fetching instruction from memory at L address
        let raw_instruction = self.read_memory(self.l)?;
        
        // decode opcode (15-11 bits) and address/argument (10-0)
        self.kod = ((raw_instruction >> 11) & 0b11111) as u8;
        self.ad = raw_instruction & 0b0000011111111111;
        
        Ok(())
    }

    /// resolves an operand address based on addressing mode
    fn _resolve_operand(&self, _instruction_code: u16) -> Result<u16, MachineError> {
        // TODO: this would analyze the instruction format
        // to determine the addressing mode and calculate the effective address
        Ok(self.ad)
    }

    pub fn step(&mut self) -> Result<(), MachineError> {
        if !self.is_running {
            return Ok(());
        }

        // checking for pending interrupts before executing instrunction 
        if self.interrupts_enabled && self.pending_interrupt.is_some() {
            let interrupt_vector = self.pending_interrupt.take().unwrap();
            
            // saving current state on stack 
            self.push_to_stack(self.ak)?;
            self.push_to_stack(self.l)?; // current L, not the incremented one
            
            // disable interrupts and jump to interrupt handler
            self.interrupts_enabled = false;
            self.l = interrupt_vector & 0b0000011111111111;
            return Ok(()); // dont execute normal instruction this cycle
        }

        self.fetch_and_decode()?;
        
        // increment instruction counter (before execution, may be overridden by jumps)
        self.l = (self.l + 1) & 0b0000011111111111;
        
        self.execute_instruction() // based on the decoded opcode
    }

    fn _handle_interrupts(&mut self) -> Result<(), MachineError> {
        if self.interrupts_enabled && self.pending_interrupt.is_some() {
            let interrupt_vector = self.pending_interrupt.take().unwrap();
            
            self.push_to_stack(self.ak)?;
            self.push_to_stack(self.l)?;
            
            self.interrupts_enabled = false;
            self.l = interrupt_vector & 0b0000011111111111;
        }
        Ok(())
    }

    pub fn trigger_interrupt(&mut self, interrupt_vector_address: u16) {
        if self.interrupts_enabled {
            self.pending_interrupt = Some(interrupt_vector_address);
        }
    }

    pub(crate) fn push_to_stack(&mut self, value: u16) -> Result<(), MachineError> {
        if self.ws == 0 {
            return Err(MachineError::StackOverflow);
        }
        self.write_memory(self.ws, value)?;
        self.ws = self.ws.wrapping_sub(1);
        Ok(())
    }

    pub(crate) fn pop_from_stack(&mut self) -> Result<u16, MachineError> {
        if self.ws >= 2047 {
            return Err(MachineError::StackUnderflow);
        }
        self.ws = self.ws.wrapping_add(1);
        let value = self.read_memory(self.ws)?;
        Ok(value)
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        self.is_running = true;
        
        while self.is_running {
            self.step()?;
        }
        
        Ok(())
    }

    pub fn run_steps(&mut self, max_steps: usize) -> Result<usize, MachineError> {
        self.is_running = true;
        let mut steps = 0;
        
        while self.is_running && steps < max_steps {
            self.step()?;
            steps += 1;
        }
        
        Ok(steps)
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

    pub fn set_input_buffer(&mut self, inputs: Vec<u16>) {
        self.input_buffer = inputs;
        self.input_buffer.reverse();
    }

    pub fn get_output_buffer(&self) -> &[u16] {
        &self.output_buffer
    }

    pub fn clear_output_buffer(&mut self) {
        self.output_buffer.clear();
    }
}
