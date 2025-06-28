use asmachina::MachineW;
use crate::error::AsmodeusError;
use crate::cli::{Args, Mode};
use crate::file_utils::{validate_file_extension, write_binary};
use crate::assembler::{assemble_file, run_program, disassemble_file, run_interactive_program};
use crate::debugger::interactive_debugger_loop;
use crate::debug_utils::{print_machine_state, print_program_loaded_banner};
use crate::ascii_art::{print_info, print_bugseer_logo};

pub fn run_mode_assemble(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified".to_string()))?;
    
    validate_file_extension(input_path, Mode::Assemble)?;
    
    let machine_code = assemble_file(input_path, args)?;
    
    if let Some(output_path) = &args.output_file {
        write_binary(output_path, &machine_code)?;
        if args.verbose {
            println!("Binary written to: {}", output_path);
        } else {
            println!("Assembly successful. Binary written to: {}", output_path);
        }
    } else {
        println!("Assembly successful!");
        println!("Machine code ({} words):", machine_code.len());
        for (i, word) in machine_code.iter().enumerate() {
            println!("  {:04X}: {:04X} ({})", i, word, word);
        }
    }
    
    Ok(())
}

pub fn run_mode_run(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified. Please provide a .asmod file to run.".to_string()))?;
    
    validate_file_extension(input_path, Mode::Run)?;
    
    if args.verbose {
        println!("Compiling and running Asmodeus program: {}", input_path);
        println!();
    }
    
    let machine_code = assemble_file(input_path, args)?;
    run_program(&machine_code, args)?;
    
    Ok(())
}

pub fn run_mode_disassemble(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified".to_string()))?;
    
    validate_file_extension(input_path, Mode::Disassemble)?;
    
    disassemble_file(input_path, args)?;
    
    Ok(())
}

pub fn run_mode_debug(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified for debug mode. Please provide a .asmod file to debug.".to_string()))?;
    
    validate_file_extension(input_path, Mode::Debug)?;
    
    if args.verbose {
        print_info(&format!("Starting Bugseer for: {}", input_path));
    }

    let machine_code = assemble_file(input_path, args)?;
    
    let mut machine = MachineW::new();
    machine.load_program(&machine_code).map_err(|e| {
        AsmodeusError::MachineError(e)
    })?;
    machine.is_running = true;

    print_bugseer_logo();
    
    print_program_loaded_banner(input_path, machine_code.len());
    
    print_machine_state(&machine);
    
    interactive_debugger_loop(&mut machine)?;
    
    Ok(())
}

pub fn run_mode_interactive(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified for interactive mode. Please provide a .asmod file to run interactively.".to_string()))?;
    
    validate_file_extension(input_path, Mode::Interactive)?;
    
    if args.verbose {
        print_info(&format!("Starting interactive mode for: {}", input_path));
        println!();
    }

    let machine_code = assemble_file(input_path, args)?;
    
    println!("\x1b[1m\x1b[38;5;39m🔤 Asmodeus Interactive Mode\x1b[0m");
    println!("Character-based I/O enabled - type characters for real-time processing");
    println!("Press Ctrl+C to interrupt\n");
    
    run_interactive_program(&machine_code, args)?;
    
    Ok(())
}
