use std::env;
use super::{Args, Mode};
use crate::error::AsmodeusError;

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
    let mut extended = false;
    let mut watch = false;
    
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
        Some("examples") => {
            mode = Mode::Examples;
            
            let sub_args: Vec<String> = args.iter().skip(2).cloned().collect();
            if !sub_args.is_empty() {
                input_file = Some(sub_args.join(" "));
            }
            
            i = args.len();
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
            "--debug-mode" => debug = true,
            "--extended" | "-e" => extended = true,
            "--watch" | "-w" => watch = true,
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
        extended,
        watch,
    })
}
