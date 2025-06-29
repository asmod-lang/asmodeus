use std::io::{self, Write};
use asmachina::{MachineW, MachineError};
use crate::error::AsmodeusError;
use crate::debug_utils::print_machine_state;
use super::{command_handlers, help};

pub fn interactive_debugger_loop(machine: &mut MachineW) -> Result<(), AsmodeusError> {
    loop {
        print!("(bugseer)> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            AsmodeusError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read input: {}", e)
            ))
        })?;
        
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        
        match command {
            "h" | "help" => help::print_debugger_help(),
            "s" | "step" => command_handlers::handle_step(machine)?,
            "n" | "next" => command_handlers::handle_next(machine)?,
            "c" | "continue" => command_handlers::handle_continue(machine)?,
            "d" | "display" => print_machine_state(machine),
            "q" | "quit" => {
                println!("Bugseer debugger terminated.");
                break;
            }
            "b" | "breakpoint" => command_handlers::handle_breakpoint(machine, &parts)?,
            "rb" | "remove-breakpoint" => command_handlers::handle_remove_breakpoint(machine, &parts)?,
            "lb" | "list-breakpoints" => command_handlers::handle_list_breakpoints(machine),
            "m" | "memory" => command_handlers::handle_memory_dump(machine, &parts)?,
            _ => println!("Unknown command: '{}'. Type 'h' for help.", command),
        }
        
        if !machine.is_running {
            if machine.kod == 0b00111 {
                println!("Program completed successfully.");
            } else {
                println!("Program halted unexpectedly.");
            }
            break;
        }
    }
    
    Ok(())
}
