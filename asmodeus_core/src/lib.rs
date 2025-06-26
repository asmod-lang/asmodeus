//! Core emulator for Machine W architecture
//! Implements the complete Von Neumann machine with 16-bit word size

use thiserror::Error;
use std::io::{self, Write};
use std::collections::HashSet;

#[derive(Error, Debug, PartialEq)]
pub enum MachineError {
    #[error("Memory address out of bounds: {address}, valid range: 0-2047")]
    AddressOutOfBounds { address: u16 },
    #[error("Invalid opcode: {opcode}")]
    InvalidOpcode { opcode: u8 },
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid addressing mode for instruction")]
    InvalidAddressingMode,
    #[error("Input/Output error: {message}")]
    IoError { message: String },
    #[error("Breakpoint hit at address {address}")]
    BreakpointHit { address: u16 },
}

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

/// Machine W architecture implementation
/// 
/// - Operating memory (PaO): 2048 words, 16-bit each
/// - Registers: AK (accumulator), L (instruction counter), AD (address), KOD (opcode), WS (stack pointer)
/// - Execution state and control flags
/// - Interrupt handling system

#[derive(Debug, Clone)]
pub struct MachineW {
    /// Operating memory - 2048 words of 16 bits each
    pub memory: Vec<u16>,
    
    /// AK - Accumulator register (16-bit)
    pub ak: u16,
    
    /// L - Instruction counter (11-bit, range 0-2047)
    pub l: u16,
    
    /// AD - Address register (11-bit)
    pub ad: u16,
    
    /// KOD - Opcode register (5-bit)
    pub kod: u8,
    
    /// WS - Stack pointer (11-bit, grows downward, initialized to 2047)
    pub ws: u16,
    
    /// Execution control flag
    pub is_running: bool,
    
    /// Interrupt control flags
    pub interrupts_enabled: bool,
    pub interrupt_mask: u16,
    
    /// Pending interrupt vector (if any)
    pub pending_interrupt: Option<u16>,
    
    /// General purpose registers (for extended addressing modes)
    pub registers: [u16; 8], // R0-R7
    
    /// I/O simulation buffers
    pub input_buffer: Vec<u16>,
    pub output_buffer: Vec<u16>,

    pub breakpoints: HashSet<u16>,
}

impl Default for MachineW {
    fn default() -> Self {
        Self::new()
    }
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
        }
    }

    /// Reads a word from memory at the specified address
    /// 
    /// # Arguments
    /// * `address` - Memory address (must be in range 0-2047)
    /// 
    /// # Returns
    /// * `Ok(u16)` - The 16-bit word at the specified address
    /// * `Err(MachineError)` - If address is out of bounds

    pub fn read_memory(&self, address: u16) -> Result<u16, MachineError> {
        let addr = address & 0b0000011111111111; // limit to 11 bits (0-2047)
        if addr >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address: addr });
        }
        Ok(self.memory[addr as usize])
    }

    /// Writes a word to memory at the specified address
    /// 
    /// # Arguments
    /// * `address` - Memory address (must be in range 0-2047)
    /// * `value` - 16-bit value to write
    /// 
    /// # Returns
    /// * `Ok(())` - If write was successful
    /// * `Err(MachineError)` - If address is out of bounds

    pub fn write_memory(&mut self, address: u16, value: u16) -> Result<(), MachineError> {
        let addr = address & 0b0000011111111111; // limit to 11 bits (0-2047)
        if addr >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address: addr });
        }
        self.memory[addr as usize] = value;
        Ok(())
    }

    /// Loads a program into memory starting at address 0
    /// 
    /// # Arguments
    /// * `program` - Slice of 16-bit instructions to load
    /// 
    /// # Returns
    /// * `Ok(())` - If program was loaded successfully
    /// * `Err(MachineError)` - If program is too large for memory

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

    /// Fetches the next instruction from memory and decodes it
    /// 
    /// # Returns
    /// * `Ok(())` - If instruction was fetched and decoded successfully
    /// * `Err(MachineError)` - If memory access failed

    fn fetch_and_decode(&mut self) -> Result<(), MachineError> {
        // fetching instruction from memory at L address
        let raw_instruction = self.read_memory(self.l)?;
        
        // decode opcode (15-11 bits) and address/argument (10-0)
        self.kod = ((raw_instruction >> 11) & 0b11111) as u8;
        self.ad = raw_instruction & 0b0000011111111111;
        
        Ok(())
    }

    /// Resolves an operand address based on the instruction's addressing mode
    /// 
    /// # Arguments
    /// * `instruction_code` - The raw instruction code to analyze for addressing mode
    /// 
    /// # Returns
    /// * `Ok(u16)` - The effective address or immediate value
    /// * `Err(MachineError)` - If addressing mode is invalid

    fn resolve_operand(&self, _instruction_code: u16) -> Result<u16, MachineError> {
        // TODO: this would analyze the instruction format
        // to determine the addressing mode and calculate the effective address
        Ok(self.ad)
    }

    /// Executes a single instruction cycle
    /// 
    /// # Returns
    /// * `Ok(())` - If instruction executed successfully
    /// * `Err(MachineError)` - If execution failed

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

    fn handle_interrupts(&mut self) -> Result<(), MachineError> {
        if self.interrupts_enabled && self.pending_interrupt.is_some() {
            let interrupt_vector = self.pending_interrupt.take().unwrap();
            
            self.push_to_stack(self.ak)?;
            self.push_to_stack(self.l)?;
            
            self.interrupts_enabled = false;
            self.l = interrupt_vector & 0b0000011111111111;
        }
        Ok(())
    }

    /// Triggers an interrupt
    /// 
    /// # Arguments
    /// * `interrupt_vector_address` - Address of the interrupt handler
    pub fn trigger_interrupt(&mut self, interrupt_vector_address: u16) {
        if self.interrupts_enabled {
            self.pending_interrupt = Some(interrupt_vector_address);
        }
    }

    fn push_to_stack(&mut self, value: u16) -> Result<(), MachineError> {
        if self.ws == 0 {
            return Err(MachineError::StackOverflow);
        }
        self.write_memory(self.ws, value)?;
        self.ws = self.ws.wrapping_sub(1);
        Ok(())
    }

    fn pop_from_stack(&mut self) -> Result<u16, MachineError> {
        if self.ws >= 2047 {
            return Err(MachineError::StackUnderflow);
        }
        self.ws = self.ws.wrapping_add(1);
        let value = self.read_memory(self.ws)?;
        Ok(value)
    }

    /// Executes the decoded instruction
    /// 
    /// # Returns
    /// * `Ok(())` - If instruction executed successfully
    /// * `Err(MachineError)` - If execution failed
    fn execute_instruction(&mut self) -> Result<(), MachineError> {
        match self.kod {
            0b00001 => self.execute_dod(), // DOD - Add
            0b00010 => self.execute_ode(), // ODE - Subtract  
            0b00011 => self.execute_lad(), // ŁAD - Store
            0b00100 => self.execute_pob(), // POB - Load
            0b00101 => self.execute_sob(), // SOB - Unconditional jump
            0b00110 => self.execute_som(), // SOM - Conditional jump
            0b10000 => self.execute_soz(), // SOZ - Jump if zero
            0b00111 => self.execute_stp(), // STP - Stop
            0b01000 => self.execute_dns(), // DNS - Disable interrupts
            0b01001 => self.execute_pzs(), // PZS - Pop from stack
            0b01010 => self.execute_sdp(), // SDP - Push to stack
            0b01011 => self.execute_czm(), // CZM - Clear interrupt mask
            0b01100 => self.execute_msk(), // MSK - Set interrupt mask
            0b01101 => self.execute_pwr(), // PWR - Return from interrupt
            0b01110 => self.execute_wejscie(), // WEJSCIE - Input
            0b01111 => self.execute_wyjscie(), // WYJSCIE - Output
            _ => Err(MachineError::InvalidOpcode { opcode: self.kod }),
        }
    }

    /// DOD - Add: (AK) + ((AD)) → AK
    fn execute_dod(&mut self) -> Result<(), MachineError> {
        let operand = self.read_memory(self.ad)?;
        self.ak = self.ak.wrapping_add(operand);
        Ok(())
    }

    /// ODE - Subtract: (AK) - ((AD)) → AK  
    fn execute_ode(&mut self) -> Result<(), MachineError> {
        let operand = self.read_memory(self.ad)?;
        self.ak = self.ak.wrapping_sub(operand);
        Ok(())
    }

    /// ŁAD - Store: (AK) → (AD)
    fn execute_lad(&mut self) -> Result<(), MachineError> {
        self.write_memory(self.ad, self.ak)
    }

    /// POB - Load: ((AD)) → AK
    fn execute_pob(&mut self) -> Result<(), MachineError> {
        self.ak = self.read_memory(self.ad)?;
        Ok(())
    }

    /// SOB - Unconditional jump: (AD) → L
    fn execute_sob(&mut self) -> Result<(), MachineError> {
        self.l = self.ad & 0b0000011111111111;
        Ok(())
    }

    /// SOM - Conditional jump: (AD) → L, when (AK) < 0
    fn execute_som(&mut self) -> Result<(), MachineError> {
        // checking if the AK value is negative 
        if (self.ak & 0x8000) != 0 {
            self.l = self.ad & 0b0000011111111111;
        }
        Ok(())
    }

    /// SOZ - Conditional jump: (AD) → L, when (AK) = 0
    fn execute_soz(&mut self) -> Result<(), MachineError> {
        // AK == zero
        if self.ak == 0 {
            self.l = self.ad & 0b0000011111111111;
        }
        Ok(())
    }

    /// STP - Stop
    fn execute_stp(&mut self) -> Result<(), MachineError> {
        self.is_running = false;
        Ok(())
    }

    /// DNS - Disable interrupts
    fn execute_dns(&mut self) -> Result<(), MachineError> {
        self.interrupts_enabled = false;
        Ok(())
    }

    /// PZS - Pop from stack: WS++, memory[WS] → AK
    fn execute_pzs(&mut self) -> Result<(), MachineError> {
        self.ak = self.pop_from_stack()?;
        Ok(())
    }

    /// SDP - Push to stack: (AK) → memory[WS], WS--
    fn execute_sdp(&mut self) -> Result<(), MachineError> {
        self.push_to_stack(self.ak)?;
        Ok(())
    }

    /// CZM - Clear interrupt mask
    fn execute_czm(&mut self) -> Result<(), MachineError> {
        self.interrupt_mask = 0;
        Ok(())
    }

    /// MSK - Set interrupt mask
    fn execute_msk(&mut self) -> Result<(), MachineError> {
        self.interrupt_mask = self.ad;
        Ok(())
    }

    /// PWR - Return from interrupt
    fn execute_pwr(&mut self) -> Result<(), MachineError> {
        // restore state from stack 
        self.l = self.pop_from_stack()?;
        self.ak = self.pop_from_stack()?;
        self.l = self.l & 0b0000011111111111;
        self.interrupts_enabled = true;
        Ok(())
    }

    /// WEJSCIE - Input operation
    fn execute_wejscie(&mut self) -> Result<(), MachineError> {
        // checking if there is data in input buffer 
        if let Some(value) = self.input_buffer.pop() {
            self.ak = value;
        } else {
            // simulating input from user (TODO) 
            print!("Input (enter a number): ");
            io::stdout().flush().map_err(|e| MachineError::IoError {
                message: format!("Failed to flush stdout: {}", e),
            })?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| MachineError::IoError {
                message: format!("Failed to read from stdin: {}", e),
            })?;
            
            let value = input.trim().parse::<u16>().map_err(|e| MachineError::IoError {
                message: format!("Invalid number format: {}", e),
            })?;
            
            self.ak = value;
        }
        Ok(())
    }

    /// WYJSCIE - Output operation
    fn execute_wyjscie(&mut self) -> Result<(), MachineError> {
        println!("Output: {}", self.ak);
        self.output_buffer.push(self.ak);
        Ok(())
    }

    /// Runs the machine until it halts or encounters an error
    /// 
    /// # Returns
    /// * `Ok(())` - If machine stopped normally (STP instruction)
    /// * `Err(MachineError)` - If execution failed
    pub fn run(&mut self) -> Result<(), MachineError> {
        self.is_running = true;
        
        while self.is_running {
            self.step()?;
        }
        
        Ok(())
    }

    /// Runs the machine for a specified number of steps or until halt
    /// 
    /// # Arguments
    /// * `max_steps` - Maximum number of steps to execute
    /// 
    /// # Returns
    /// * `Ok(usize)` - Number of steps actually executed
    /// * `Err(MachineError)` - If execution failed
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

    /// === DEBUGGING SUPPORT METHODS ===
    /// Execute a single instruction (for step debugging)
    pub fn step_instruction(&mut self) -> Result<(), MachineError> {
        if !self.is_running {
            return Ok(());
        }

        if self.breakpoints.contains(&self.l) {
            return Err(MachineError::BreakpointHit { address: self.l });
        }

        self.step()
    }

    pub fn run_until_halt_or_breakpoint(&mut self) -> Result<(), MachineError> {
        while self.is_running {

            if self.breakpoints.contains(&self.l) {
                return Err(MachineError::BreakpointHit { address: self.l });
            }

            self.step()?;
        }
        Ok(())
    }

    /// snapshot
    pub fn get_current_state(&self) -> MachineWState {
        MachineWState {
            ak: self.ak,
            l: self.l,
            ad: self.ad,
            kod: self.kod,
            ws: self.ws,
            is_running: self.is_running,
            interrupts_enabled: self.interrupts_enabled,
            interrupt_mask: self.interrupt_mask,
            registers: self.registers,
        }
    }

    pub fn get_memory_range(&self, start_addr: u16, end_addr: u16) -> Option<Vec<(u16, u16)>> {
        if start_addr > end_addr || end_addr >= 2048 {
            return None;
        }

        let mut result = Vec::new();
        for addr in start_addr..=end_addr {
            result.push((addr, self.memory[addr as usize]));
        }
        Some(result)
    }

    pub fn add_breakpoint(&mut self, address: u16) -> Result<(), MachineError> {
        if address >= 2048 {
            return Err(MachineError::AddressOutOfBounds { address });
        }
        self.breakpoints.insert(address);
        Ok(())
    }

    pub fn has_breakpoint(&self, address: u16) -> bool {
        self.breakpoints.contains(&address)
    }

    pub fn remove_breakpoint(&mut self, address: u16) -> bool {
        self.breakpoints.remove(&address)
    }

    pub fn list_breakpoints(&self) -> Vec<u16> {
        let mut breakpoints: Vec<u16> = self.breakpoints.iter().copied().collect();
        breakpoints.sort();
        breakpoints
    }

    pub fn clear_all_breakpoints(&mut self) {
        self.breakpoints.clear();
    }
}
