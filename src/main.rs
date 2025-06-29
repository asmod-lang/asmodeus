use std::process;

mod error;
mod cli;
mod file_utils;
mod debug;
mod assembler;
mod bugseer;
mod modes;
mod ascii_art;
mod examples_manager;

use cli::{parse_args, print_help, Mode};
use modes::{run_mode_assemble, run_mode_run, run_mode_disassemble, run_mode_debug, run_mode_interactive};

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Use --help for usage information.");
            process::exit(1);
        }
    };

    let result = match args.mode {
        Mode::Help => {
            print_help();
            Ok(())
        }
        Mode::Assemble => run_mode_assemble(&args),
        Mode::Run => run_mode_run(&args),
        Mode::Debug => run_mode_debug(&args),
        Mode::Interactive => run_mode_interactive(&args),
        Mode::Disassemble => run_mode_disassemble(&args),
        Mode::Examples => examples_manager::handle_examples_command(&args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
