use asmachina::{MachineW, MachineError};
use crate::error::AsmodeusError;
use crate::debug_utils::print_machine_state;
use super::address_parser::parse_address;

pub fn handle_step(machine: &mut MachineW) -> Result<(), AsmodeusError> {
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

pub fn handle_next(machine: &mut MachineW) -> Result<(), AsmodeusError> {
    handle_step(machine)
}

pub fn handle_continue(machine: &mut MachineW) -> Result<(), AsmodeusError> {
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

pub fn handle_breakpoint(machine: &mut MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
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

pub fn handle_remove_breakpoint(machine: &mut MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
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

pub fn handle_list_breakpoints(machine: &MachineW) {
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

pub fn handle_memory_dump(machine: &MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
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
