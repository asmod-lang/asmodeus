use std::io::{self, Write};
use asmachina::{MachineW, MachineError};
use crate::error::AsmodeusError;
use crate::debug_utils::print_machine_state;

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
            "h" | "help" => print_debugger_help(),
            "s" | "step" => handle_step(machine)?,
            "n" | "next" => handle_next(machine)?,
            "c" | "continue" => handle_continue(machine)?,
            "d" | "display" => print_machine_state(machine),
            "q" | "quit" => {
                println!("Debugger terminated.");
                break;
            }
            "b" | "breakpoint" => handle_breakpoint(machine, &parts)?,
            "rb" | "remove-breakpoint" => handle_remove_breakpoint(machine, &parts)?,
            "lb" | "list-breakpoints" => handle_list_breakpoints(machine),
            "m" | "memory" => handle_memory_dump(machine, &parts)?,
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

pub fn print_debugger_help() {
    println!("Available commands:");
    println!("  s, step           - Execute one instruction (F7)");
    println!("  n, next           - Execute one instruction (F9) [alias for step]");
    println!("  c, continue       - Continue execution until halt or breakpoint (F10)");
    println!("  d, display        - Display current machine state");
    println!("  b <addr>          - Set breakpoint at address (decimal or hex)");
    println!("  rb <addr>         - Remove breakpoint at address");
    println!("  lb                - List all breakpoints");
    println!("  m <start> [end]   - Memory dump (16 words from start, or range)");
    println!("  q, quit           - Quit debugger");
    println!("  h, help           - Show this help");
    println!();
}

fn handle_step(machine: &mut MachineW) -> Result<(), AsmodeusError> {
    match machine.step_instruction() {
        Ok(()) => {
            println!("Step executed.");
            print_machine_state(machine);
        }
        Err(MachineError::BreakpointHit { address }) => {
            println!("Breakpoint hit at address {}!", address);
            print_machine_state(machine);
        }
        Err(e) => {
            println!("Execution error: {}", e);
        }
    }
    Ok(())
}

fn handle_next(machine: &mut MachineW) -> Result<(), AsmodeusError> {
    handle_step(machine)
}

fn handle_continue(machine: &mut MachineW) -> Result<(), AsmodeusError> {
    match machine.run_until_halt_or_breakpoint() {
        Ok(()) => {
            println!("Program completed successfully.");
            print_machine_state(machine);
        }
        Err(MachineError::BreakpointHit { address }) => {
            println!("Breakpoint hit at address {}!", address);
            print_machine_state(machine);
        }
        Err(e) => {
            println!("Execution error: {}", e);
        }
    }
    Ok(())
}

fn handle_breakpoint(machine: &mut MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
    if parts.len() != 2 {
        println!("Usage: b <address>");
        return Ok(());
    }
    
    let address_str = parts[1];
    let address = parse_address(address_str)?;
    
    match machine.add_breakpoint(address) {
        Ok(()) => println!("Breakpoint set at address {}", address),
        Err(e) => println!("Failed to set breakpoint: {}", e),
    }
    
    Ok(())
}

fn handle_remove_breakpoint(machine: &mut MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
    if parts.len() != 2 {
        println!("Usage: rb <address>");
        return Ok(());
    }
    
    let address_str = parts[1];
    let address = parse_address(address_str)?;
    
    if machine.remove_breakpoint(address) {
        println!("Breakpoint removed from address {}", address);
    } else {
        println!("No breakpoint at address {}", address);
    }
    
    Ok(())
}

fn handle_list_breakpoints(machine: &MachineW) {
    let breakpoints = machine.list_breakpoints();
    if breakpoints.is_empty() {
        println!("No breakpoints set.");
    } else {
        println!("Breakpoints:");
        for addr in breakpoints {
            println!("  {}", addr);
        }
    }
}

fn handle_memory_dump(machine: &MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
    if parts.len() < 2 {
        println!("Usage: m <start_addr> [end_addr]");
        return Ok(());
    }
    
    let start_addr = parse_address(parts[1])?;
    let end_addr = if parts.len() >= 3 {
        parse_address(parts[2])?
    } else {
        (start_addr + 15).min(2047) // 16 words or until end of memory
    };
    
    if let Some(memory_range) = machine.get_memory_range(start_addr, end_addr) {
        println!("Memory dump:");
        for (addr, value) in memory_range {
            println!("  {:04}: {:04X} ({})", addr, value, value);
        }
    } else {
        println!("Invalid memory range: {} to {}", start_addr, end_addr);
    }
    
    Ok(())
}

fn parse_address(addr_str: &str) -> Result<u16, AsmodeusError> {
    if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
        // hex
        u16::from_str_radix(&addr_str[2..], 16).map_err(|_| {
            AsmodeusError::UsageError(format!("Invalid hexadecimal address: {}", addr_str))
        })
    } else {
        // decimal
        addr_str.parse::<u16>().map_err(|_| {
            AsmodeusError::UsageError(format!("Invalid decimal address: {}", addr_str))
        })
    }
}
