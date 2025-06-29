use asmachina::MachineW;
use crate::error::AsmodeusError;
use crate::cli::Args;
use crate::debug::{print_machine_state, print_program_output};
use crate::ascii_art::{print_success, print_info};

pub fn run_program(machine_code: &[u16], args: &Args) -> Result<(), AsmodeusError> {
    if args.verbose {
        print_info("Creating Asmachina emulator...");
    }
    
    let mut machine = MachineW::new();
    
    if args.verbose {
        print_info("Loading program into memory...");
    }
    
    machine.load_program(machine_code)?;
    
    if args.verbose {
        print_info("Starting execution...");
        println!();
    }
    
    machine.run().map_err(|e| {
        AsmodeusError::MachineError(e)
    })?;
    
    print_success("Program execution completed successfully.");
    println!();
    print_machine_state(&machine);
    
    let output_buffer = machine.get_output_buffer();
    print_program_output(output_buffer);
    
    Ok(())
}
