use asmachina::MachineW;
use crate::error::AsmodeusError;
use crate::cli::{Args, Mode};
use crate::file_utils::{validate_file_extension, write_binary};
use crate::assembler::{assemble_file, run_program, disassemble_file, run_interactive_program};
use crate::bugseer::interactive_debugger_loop;
use crate::debug::{print_machine_state, print_program_loaded_banner};
use crate::ascii_art::{print_info, print_bugseer_logo};
use std::time::Duration;
use std::thread;
use std::fs::metadata;

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
    if args.watch {
        return run_mode_watch(args);
    }
    
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified".to_string()))?;
    
    validate_file_extension(input_path, Mode::Run)?;
    
    let machine_code = assemble_file(input_path, args)?;
    run_program(&machine_code, args)
}

pub fn run_mode_watch(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified for watch mode".to_string()))?;
    
    validate_file_extension(input_path, Mode::Run)?;
    
    println!("👁️  Watch mode: Monitoring {} for changes...", input_path);
    println!("Press Ctrl+C to stop watching\n");
    
    let mut last_modified = metadata(input_path)
        .map_err(|e| AsmodeusError::IoError(e))?
        .modified()
        .map_err(|e| AsmodeusError::IoError(e))?;
    
    println!("🚀 Initial run:");
    run_program_once(input_path, args)?;
    
    loop {
        thread::sleep(Duration::from_millis(500));
        
        if let Ok(meta) = metadata(input_path) {
            if let Ok(modified) = meta.modified() {
                if modified > last_modified {
                    last_modified = modified;
                    println!("\n🔄 File changed, recompiling and running...\n");
                    
                    match run_program_once(input_path, args) {
                        Ok(_) => println!("✅ Program executed successfully"),
                        Err(e) => println!("❌ Error: {}", e),
                    }
                    println!("\n👁️  Continuing to watch for changes...\n");
                }
            }
        }
    }
}

fn run_program_once(input_path: &str, args: &Args) -> Result<(), AsmodeusError> {
    let mut run_args = args.clone();
    run_args.watch = false;
    
    let machine_code = assemble_file(input_path, &run_args)?;
    run_program(&machine_code, &run_args)
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
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified for interactive mode".to_string()))?;
    
    validate_file_extension(input_path, Mode::Interactive)?;
    
    let machine_code = assemble_file(input_path, args)?;
    run_interactive_program(&machine_code, args)
}
