//! command line interface and argument parsing

use std::env;
use crate::error::AsmodeusError;

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
    println!("Asmodeus - Machine W Emulator and Assembler");
    println!("Usage: asmod <COMMAND> [OPTIONS] <INPUT_FILE>");
    println!("       asmod [OPTIONS] <INPUT_FILE>  (defaults to run)");
    println!();
    println!("COMMANDS:");
    println!("  run           Run the assembly program (default)");
    println!("  assemble      Assemble to binary without running");
    println!("  disassemble   Disassemble binary file");
    println!("  debug         Interactive debugger with breakpoints");
    println!("  interactive   Real-time character I/O mode");
    println!("  live          Alias for interactive mode");
    println!();
    println!("OPTIONS:");
    println!("  -o, --output      Specify output file");
    println!("  -v, --verbose     Verbose output");
    println!("  --debug           Debug output");
    println!("  -h, --help        Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  asmod run program.asmod           # Run assembly program");
    println!("  asmod run --debug program.asmod   # Run with debug output");
    println!("  asmod debug program.asmod         # Interactive debugger");
    println!("  asmod interactive char_io.asmod   # Real-time character I/O");
    println!("  asmod assemble program.asmod      # Assemble to binary");
    println!("  asmod disassemble program.bin     # Disassemble binary");
    println!("  asmod program.asmod               # Run (default command)");
    println!();
    println!("SUPPORTED FILE EXTENSIONS:");
    println!("  .asmod    Asmodeus assembly source files");
    println!("  .asm      Alternative assembly source files"); 
    println!("  .bin      Binary machine code files");
    println!();
    println!("🚀 Asmodeus - Your Machine W Development Environment");
}
