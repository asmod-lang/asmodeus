use std::env;
use crate::error::AsmodeusError;
use crate::ascii_art::{print_asmodeus_logo_full, print_command, print_info};

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Assemble,
    Disassemble,
    Run,
    Debug,
    Interactive,
    Help,
}

#[derive(Debug)]
pub struct Args {
    pub mode: Mode,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub verbose: bool,
    pub debug: bool,
}

pub fn parse_args() -> Result<Args, AsmodeusError> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err(AsmodeusError::UsageError("No arguments provided".to_string()));
    }

    let mut mode = Mode::Run;
    let mut input_file = None;
    let mut output_file = None;
    let mut verbose = false;
    let mut debug = false;
    
    let mut i;
    
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => {
            mode = Mode::Run;
            i = 2; // skip subcommand
        }
        Some("assemble") => {
            mode = Mode::Assemble;
            i = 2;
        }
        Some("disassemble") => {
            mode = Mode::Disassemble;
            i = 2;
        }
        Some("debug") => {
            mode = Mode::Debug;
            i = 2;
        }
        Some("interactive") | Some("live") => {
            mode = Mode::Interactive;
            i = 2;
        }
        Some("--help") | Some("-h") => {
            mode = Mode::Help;
            i = 2;
        }
        Some(arg) if !arg.starts_with('-') => {
            // no subcommand = first arg as filename and default to run
            mode = Mode::Run;
            input_file = Some(arg.to_string());
            i = 2;
        }
        _ => {
            // old parsing
            i = 1;
        }
    }
    
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                mode = Mode::Help;
                break;
            }
            "--assemble" | "-a" => mode = Mode::Assemble,
            "--disassemble" | "-d" => mode = Mode::Disassemble,
            "--run" | "-r" => mode = Mode::Run,
            "--output" | "-o" => {
                i += 1;
                if i < args.len() {
                    output_file = Some(args[i].clone());
                } else {
                    return Err(AsmodeusError::UsageError("Missing output file".to_string()));
                }
            }
            "--verbose" | "-v" => verbose = true,
            "--debug" => debug = true,
            arg if arg.starts_with('-') => {
                return Err(AsmodeusError::UsageError(format!("Unknown option: {}", arg)));
            }
            _ => {
                if input_file.is_none() {
                    input_file = Some(args[i].clone());
                } else {
                    return Err(AsmodeusError::UsageError("Multiple input files specified".to_string()));
                }
            }
        }
        i += 1;
    }

    Ok(Args {
        mode,
        input_file,
        output_file,
        verbose,
        debug,
    })
}

pub fn print_help() {
    print_asmodeus_logo_full();
    
    println!("Usage: asmod <COMMAND> [OPTIONS] <INPUT_FILE>");
    println!("       asmod [OPTIONS] <INPUT_FILE>  (defaults to run)");
    println!();
    
    println!("COMMANDS:");
    print_command("run", "Run the assembly program (default)");
    print_command("assemble", "Assemble to binary without running");
    print_command("disassemble", "Disassemble binary file");
    print_command("debug", "Interactive debugger with breakpoints");
    print_command("interactive", "Real-time character I/O mode");
    print_command("live", "Alias for interactive mode");
    println!();
    
    println!("OPTIONS:");
    print_command("-o, --output", "Specify output file");
    print_command("-v, --verbose", "Verbose output");
    print_command("--debug", "Debug output");
    print_command("-h, --help", "Show this help message");
    println!();
    
    println!("EXAMPLES:");
    print_command("asmod run program.asmod", "# Run assembly program");
    print_command("asmod run --debug program.asmod", "# Run with debug output");
    print_command("asmod debug program.asmod", "# Interactive debugger");
    print_command("asmod interactive char_io.asmod", "# Real-time character I/O");
    print_command("asmod assemble program.asmod", "# Assemble to binary");
    print_command("asmod disassemble program.bin", "# Disassemble binary");
    print_command("asmod program.asmod", "# Run (default command)");
    println!();
    
    println!("SUPPORTED FILE EXTENSIONS:");
    print_info(".asmod    Asmodeus assembly source files");
    print_info(".asm      Alternative assembly source files"); 
    print_info(".bin      Binary machine code files");
    println!();
    
    println!("\x1b[1m\x1b[38;5;202m🚀 Asmodeus - Your Machine W Development Environment\x1b[0m");
}
