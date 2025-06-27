//! assembly and disassembly operations

use std::fs;
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;
use dismael::disassemble;
use asmachina::MachineW;

use crate::error::AsmodeusError;
use crate::cli::Args;
use crate::file_utils::read_file;
use crate::debug_utils::{print_tokens_debug, print_ast_debug, print_machine_state};

pub fn assemble_file(input_path: &str, args: &Args) -> Result<Vec<u16>, AsmodeusError> {
    if args.verbose {
        println!("Reading source file: {}", input_path);
    }
    
    let source = read_file(input_path)?;
    
    if args.verbose {
        println!("Tokenizing source code...");
    }
    
    let tokens = tokenize(&source)?;
    
    if args.debug {
        print_tokens_debug(&tokens);
    }
    
    if args.verbose {
        println!("Parsing tokens to AST...");
    }
    
    let ast = parse(tokens)?;
    
    if args.debug {
        print_ast_debug(&ast);
    }
    
    if args.verbose {
        println!("Assembling AST to machine code...");
    }
    
    let machine_code = assemble_program(&ast)?;
    
    if args.verbose {
        println!("Generated {} words of machine code", machine_code.len());
    }
    
    Ok(machine_code)
}

pub fn run_program(machine_code: &[u16], args: &Args) -> Result<(), AsmodeusError> {
    if args.verbose {
        println!("Creating Machine W emulator...");
    }
    
    let mut machine = MachineW::new();
    
    if args.verbose {
        println!("Loading program into memory...");
    }
    
    machine.load_program(machine_code)?;
    
    if args.verbose {
        println!("Starting execution...");
        println!("Initial machine state:");
        print_machine_state(&machine);
        println!();
    }
    
    machine.run().map_err(|_e| {
        AsmodeusError::UsageError("Program execution failed".to_string())
    })?;
    
    println!("Program execution completed successfully.");
    println!();
    println!("Final machine state:");
    print_machine_state(&machine);
    
    let output_buffer = machine.get_output_buffer();
    if !output_buffer.is_empty() {
        println!();
        println!("Program output:");
        for (i, &value) in output_buffer.iter().enumerate() {
            println!("  [{}] {} (0x{:04X})", i, value, value);
        }
    }
    
    Ok(())
}

pub fn disassemble_file(input_path: &str, args: &Args) -> Result<(), AsmodeusError> {
    use crate::file_utils::read_binary;
    
    if args.verbose {
        println!("Reading binary file: {}", input_path);
    }
    
    let machine_code = read_binary(input_path)?;
    
    if args.verbose {
        println!("Disassembling {} words of machine code...", machine_code.len());
    }
    
    let assembly = disassemble(&machine_code)?;
    
    let output = assembly.join("\n");
    
    if let Some(output_path) = &args.output_file {
        fs::write(output_path, output).map_err(|e| {
            AsmodeusError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to write output file '{}': {}", output_path, e)
            ))
        })?;
        if args.verbose {
            println!("Disassembly written to: {}", output_path);
        }
    } else {
        println!("{}", output);
    }
    
    Ok(())
}

pub fn run_interactive_program(machine_code: &[u16], args: &Args) -> Result<(), AsmodeusError> {
    println!("🔤 Asmodeus Interactive Mode");
    println!("Character-based I/O enabled - type characters for real-time processing");
    println!("Press Ctrl+C to interrupt\n");
    
    let mut machine = MachineW::new();
    machine.set_interactive_mode(true);
    machine.load_program(machine_code)?;
    
    if args.verbose {
        println!("Program loaded: {} words", machine_code.len());
        println!("Interactive character I/O mode: ON");
        println!("Program starting...\n");
    }
    
    match machine.run() {
        Ok(_) => {
            println!("\n✅ Program completed successfully.");
            if args.verbose {
                println!("Final machine state:");
                println!("AK: {:04X} ({})", machine.ak, machine.ak);
                if !machine.get_output_buffer().is_empty() {
                    println!("Output buffer: {:?}", machine.get_output_buffer());
                }
            }
        }
        Err(e) => {
            eprintln!("\n❌ Execution error: {}", e);
            return Err(AsmodeusError::MachineError(e));
        }
    }
    
    Ok(())
}
