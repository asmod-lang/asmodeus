//! all instruction implementations

use crate::error::MachineError;
use crate::machine::MachineW;
use std::io::{self, Write, Read};

impl MachineW {

    pub(crate) fn execute_instruction(&mut self) -> Result<(), MachineError> {
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
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let effective_address = self.resolve_effective_address(raw_instruction)?;
        let operand = self.read_memory(effective_address)?;
        self.ak = self.ak.wrapping_add(operand);
        Ok(())
    }

    /// ODE - Subtract: (AK) - ((AD)) → AK  
    fn execute_ode(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let effective_address = self.resolve_effective_address(raw_instruction)?;
        let operand = self.read_memory(effective_address)?;
        self.ak = self.ak.wrapping_sub(operand);
        Ok(())
    }

    /// ŁAD - Store: (AK) → (AD)
    fn execute_lad(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let effective_address = self.resolve_effective_address(raw_instruction)?;
        self.write_memory(effective_address, self.ak)
    }

    /// POB - Load: ((AD)) → AK
    fn execute_pob(&mut self) -> Result<(), MachineError> {
        let raw_instruction = self.memory[self.l.wrapping_sub(1) as usize];
        let effective_address = self.resolve_effective_address(raw_instruction)?;
        self.ak = self.read_memory(effective_address)?;
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
        if self.interactive_mode {
            // interactive character input mode
            let mut buffer = [0; 1];
            match io::stdin().read_exact(&mut buffer) {
                Ok(_) => {
                    self.ak = buffer[0] as u16; // ASCII value
                }
                Err(e) => {
                    return Err(MachineError::IoError {
                        message: format!("Failed to read character: {}", e),
                    });
                }
            }
        } else {
            // backward compatibility
            if let Some(value) = self.input_buffer.pop() {
                self.ak = value;
            } else {
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
        }
        Ok(())
    }

    /// WYJSCIE - Output operation
    fn execute_wyjscie(&mut self) -> Result<(), MachineError> {
        if self.interactive_mode {
            // interactive character output mode
            let byte_value = (self.ak & 0xFF) as u8; // lower 8 bits
            if byte_value >= 32 && byte_value <= 126 {
                // printable ASCII character
                print!("{}", byte_value as char);
            } else if byte_value == 10 {
                // newline
                println!();
            } else {
                // non-printable, show as number
                print!("[{}]", byte_value);
            }
            io::stdout().flush().map_err(|e| MachineError::IoError {
                message: format!("Failed to flush stdout: {}", e),
            })?;
        }

        self.output_buffer.push(self.ak);
        Ok(())
    }
}
