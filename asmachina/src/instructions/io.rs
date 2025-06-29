use crate::error::MachineError;
use crate::machine::MachineW;
use std::io::{self, Write, Read};

impl MachineW {
    /// WEJSCIE - Input operation
    pub(crate) fn execute_wejscie(&mut self) -> Result<(), MachineError> {
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
    pub(crate) fn execute_wyjscie(&mut self) -> Result<(), MachineError> {
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
