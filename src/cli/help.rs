use crate::ascii_art::{print_asmodeus_logo_full, print_command, print_info};

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
