//! Asmodeus - Machine W Emulator and Assembler

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use asmodeus_core::{MachineW, MachineError};
use std::io::{self, Write};
use asmodeus_lexer::{tokenize, Token};
use asmodeus_parser::{parse, ast::{Program, ProgramElement}};
use asmodeus_assembler::assemble_program;
use asmodeus_disassembler::disassemble;

#[derive(Debug)]
enum AsmodeusError {
    IoError(std::io::Error),
    LexerError(asmodeus_lexer::LexerError),
    ParserError(asmodeus_parser::ParserError),
    AssemblerError(asmodeus_assembler::AssemblerError),
    MachineError(asmodeus_core::MachineError),
    DisassemblerError(asmodeus_disassembler::DisassemblerError),
    UsageError(String),
}

impl std::fmt::Display for AsmodeusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmodeusError::IoError(e) => write!(f, "I/O Error: {}", e),
            AsmodeusError::LexerError(e) => write!(f, "Lexer Error: {}", e),
            AsmodeusError::ParserError(e) => write!(f, "Parser Error: {}", e),
            AsmodeusError::AssemblerError(e) => write!(f, "Assembler Error: {}", e),
            AsmodeusError::MachineError(e) => write!(f, "Machine Error: {}", e),
            AsmodeusError::DisassemblerError(e) => write!(f, "Disassembler Error: {}", e),
            AsmodeusError::UsageError(e) => write!(f, "Usage Error: {}", e),
        }
    }
}

impl std::error::Error for AsmodeusError {}

impl From<std::io::Error> for AsmodeusError {
    fn from(error: std::io::Error) -> Self {
        AsmodeusError::IoError(error)
    }
}

impl From<asmodeus_lexer::LexerError> for AsmodeusError {
    fn from(error: asmodeus_lexer::LexerError) -> Self {
        AsmodeusError::LexerError(error)
    }
}

impl From<asmodeus_parser::ParserError> for AsmodeusError {
    fn from(error: asmodeus_parser::ParserError) -> Self {
        AsmodeusError::ParserError(error)
    }
}

impl From<asmodeus_assembler::AssemblerError> for AsmodeusError {
    fn from(error: asmodeus_assembler::AssemblerError) -> Self {
        AsmodeusError::AssemblerError(error)
    }
}

impl From<asmodeus_core::MachineError> for AsmodeusError {
    fn from(error: asmodeus_core::MachineError) -> Self {
        AsmodeusError::MachineError(error)
    }
}

impl From<asmodeus_disassembler::DisassemblerError> for AsmodeusError {
    fn from(error: asmodeus_disassembler::DisassemblerError) -> Self {
        AsmodeusError::DisassemblerError(error)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Assemble,
    Disassemble,
    Run,
    Debug,
    Help,
}

#[derive(Debug)]
struct Args {
    mode: Mode,
    input_file: Option<String>,
    output_file: Option<String>,
    verbose: bool,
    debug: bool,
}

fn parse_args() -> Result<Args, AsmodeusError> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err(AsmodeusError::UsageError("No arguments provided".to_string()));
    }

    let mut mode = Mode::Run;
    let mut input_file = None;
    let mut output_file = None;
    let mut verbose = false;
    let mut debug = false;
    
    let mut i = 1;
    
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

fn print_help() {
    println!("Asmodeus - Machine W Emulator and Assembler");
    println!("Usage: asmod <COMMAND> [OPTIONS] <INPUT_FILE>");
    println!("       asmod [OPTIONS] <INPUT_FILE>  (defaults to run)");
    println!();
    println!("COMMANDS:");
    println!("  run           Run the assembly program (default)");
    println!("  assemble      Assemble to binary without running");
    println!("  disassemble   Disassemble binary file");
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

fn validate_file_extension(path: &str, mode: Mode) -> Result<(), AsmodeusError> {
    let path = Path::new(path);
    let extension = path.extension().and_then(|ext| ext.to_str());
    
    match (mode.clone(), extension) {
        (Mode::Run | Mode::Assemble | Mode::Debug, Some("asmod")) => Ok(()),
        (Mode::Run | Mode::Assemble | Mode::Debug, Some("asm")) => Ok(()),
        (Mode::Disassemble, Some("bin")) => Ok(()),
        (Mode::Help, _) => Ok(()), // Help mode doesn't need file validation
        (Mode::Run | Mode::Assemble | Mode::Debug, Some(ext)) => {
            Err(AsmodeusError::UsageError(
                format!("Expected .asmod or .asm file, but got .{} file. Please use a valid Asmodeus source file.", ext)
            ))
        }
        (Mode::Disassemble, Some(ext)) => {
            Err(AsmodeusError::UsageError(
                format!("Expected .bin file for disassembly, but got .{} file.", ext)
            ))
        }
        (_, None) => {
            Err(AsmodeusError::UsageError(
                format!("File has no extension. Expected: .asmod/.asm for run/assemble/debug, .bin for disassemble")
            ))
        }
    }
}

fn read_file(path: &str) -> Result<String, AsmodeusError> {
    fs::read_to_string(path).map_err(|e| {
        AsmodeusError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to read file '{}': {}", path, e)
        ))
    })
}

fn write_binary(path: &str, data: &[u16]) -> Result<(), AsmodeusError> {
    let bytes: Vec<u8> = data.iter()
        .flat_map(|&word| word.to_le_bytes())
        .collect();
    fs::write(path, bytes).map_err(|e| {
        AsmodeusError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to write binary file '{}': {}", path, e)
        ))
    })
}

fn read_binary(path: &str) -> Result<Vec<u16>, AsmodeusError> {
    let bytes = fs::read(path).map_err(|e| {
        AsmodeusError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to read binary file '{}': {}", path, e)
        ))
    })?;
    
    if bytes.len() % 2 != 0 {
        return Err(AsmodeusError::UsageError(
            format!("Binary file '{}' has odd number of bytes ({})", path, bytes.len())
        ));
    }
    
    let words: Vec<u16> = bytes.chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    
    Ok(words)
}

fn print_tokens_debug(tokens: &[Token]) {
    println!("=== TOKENS ===");
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {} {{", i + 1);
        println!("    kind: {:?},", token.kind);
        println!("    value: \"{}\",", token.value);
        println!("    line: {},", token.line);
        println!("    column: {}", token.column);
        println!("}},");
    }
    println!("======\n");
}

fn print_ast_debug(ast: &Program) {
    println!("=== AST ===");
    println!("Program {{");
    println!("    elements: [");
    for (i, element) in ast.elements.iter().enumerate() {
        print!("        ");
        match element {
            ProgramElement::LabelDefinition(label) => {
                println!("LabelDefinition(");
                println!("            LabelDefinition {{");
                println!("                name: \"{}\",", label.name);
                println!("                line: {},", label.line);
                println!("                column: {}", label.column);
                println!("            }}");
                print!("        )");
            },
            ProgramElement::Instruction(instr) => {
                println!("Instruction(");
                println!("            Instruction {{");
                println!("                opcode: \"{}\",", instr.opcode);
                match &instr.operand {
                    Some(operand) => {
                        println!("                operand: Some(Operand {{");
                        println!("                    addressing_mode: {:?},", operand.addressing_mode);
                        println!("                    value: \"{}\"", operand.value);
                        println!("                }}),");
                    },
                    None => println!("                operand: None,"),
                }
                println!("                line: {},", instr.line);
                println!("                column: {}", instr.column);
                println!("            }}");
                print!("        )");
            },
            ProgramElement::Directive(directive) => {
                println!("Directive(");
                println!("            Directive {{");
                println!("                name: \"{}\",", directive.name);
                println!("                arguments: {:?},", directive.arguments);
                println!("                line: {},", directive.line);
                println!("                column: {}", directive.column);
                println!("            }}");
                print!("        )");
            }
            ProgramElement::MacroDefinition(_) => {
                println!("MacroDefinition(...)");
                print!("        ");
            }
            ProgramElement::MacroCall(_) => {
                println!("MacroCall(...)");
                print!("        ");
            }
        }
        if i < ast.elements.len() - 1 {
            println!(",");
        } else {
            println!();
        }
    }
    println!("    ]");
    println!("}}");
    println!("======\n");
}

fn assemble_file(input_path: &str, args: &Args) -> Result<Vec<u16>, AsmodeusError> {
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

fn run_program(machine_code: &[u16], args: &Args) -> Result<(), AsmodeusError> {
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

fn print_machine_state(machine: &MachineW) {
    let state = machine.get_current_state();
    println!("=== MACHINE STATE ===");
    println!("AK: {:04X} ({})    L: {:04X} ({})    AD: {:04X} ({})", 
             state.ak, state.ak, state.l, state.l, state.ad, state.ad);
    println!("KOD: {:02X} ({})      WS: {:04X} ({})    Running: {}", 
             state.kod, state.kod, state.ws, state.ws, state.is_running);
    println!("Interrupts: {}    Mask: {:04X}", 
             state.interrupts_enabled, state.interrupt_mask);
    println!("======================");
}

fn disassemble_file(input_path: &str, args: &Args) -> Result<(), AsmodeusError> {
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

fn run_mode_assemble(args: &Args) -> Result<(), AsmodeusError> {
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

fn run_mode_run(args: &Args) -> Result<(), AsmodeusError> {
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

fn run_mode_disassemble(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified".to_string()))?;
    
    validate_file_extension(input_path, Mode::Disassemble)?;
    
    disassemble_file(input_path, args)?;
    
    Ok(())
}

fn run_mode_debug(args: &Args) -> Result<(), AsmodeusError> {
    let input_path = args.input_file.as_ref()
        .ok_or_else(|| AsmodeusError::UsageError("No input file specified for debug mode. Please provide a .asmod file to debug.".to_string()))?;
    
    validate_file_extension(input_path, Mode::Debug)?;
    
    if args.verbose {
        println!("🐛 Starting interactive debugger for: {}", input_path);
        println!();
    }

    let machine_code = assemble_file(input_path, args)?;
    
    let mut machine = MachineW::new();
    machine.load_program(&machine_code).map_err(|e| {
        AsmodeusError::MachineError(e)
    })?;
    machine.is_running = true;

    println!("🐛 Asmodeus Interactive Debugger");
    println!("Program loaded: {} ({} words)", input_path, machine_code.len());
    println!("Type 'h' for help\n");

    print_machine_state(&machine);
    interactive_debugger_loop(&mut machine)?;
    
    Ok(())
}

fn interactive_debugger_loop(machine: &mut MachineW) -> Result<(), AsmodeusError> {
    loop {
        print!("(asmod-debug)> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            AsmodeusError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read input: {}", e)
            ))
        })?;
        
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        
        match command {
            "h" | "help" => print_debugger_help(),
            "s" | "step" => handle_step(machine)?,
            "n" | "next" => handle_next(machine)?,
            "c" | "continue" => handle_continue(machine)?,
            "d" | "display" => print_machine_state(machine),
            "q" | "quit" => {
                println!("Debugger terminated.");
                break;
            }
            "b" | "breakpoint" => handle_breakpoint(machine, &parts)?,
            "rb" | "remove-breakpoint" => handle_remove_breakpoint(machine, &parts)?,
            "lb" | "list-breakpoints" => handle_list_breakpoints(machine),
            "m" | "memory" => handle_memory_dump(machine, &parts)?,
            _ => println!("Unknown command: '{}'. Type 'h' for help.", command),
        }
        
        if !machine.is_running {
            if machine.kod == 0b00111 {
                println!("Program completed successfully.");
            } else {
                println!("Program halted unexpectedly.");
            }
            break;
        }
    }
    
    Ok(())
}

fn print_debugger_help() {
    println!("Available commands:");
    println!("  s, step           - Execute one instruction (F7)");
    println!("  n, next           - Execute one instruction (F9) [alias for step]");
    println!("  c, continue       - Continue execution until halt or breakpoint (F10)");
    println!("  d, display        - Display current machine state");
    println!("  b <addr>          - Set breakpoint at address (decimal or hex)");
    println!("  rb <addr>         - Remove breakpoint at address");
    println!("  lb                - List all breakpoints");
    println!("  m <start> [end]   - Memory dump (16 words from start, or range)");
    println!("  q, quit           - Quit debugger");
    println!("  h, help           - Show this help");
    println!();
}

fn handle_step(machine: &mut MachineW) -> Result<(), AsmodeusError> {
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

fn handle_next(machine: &mut MachineW) -> Result<(), AsmodeusError> {
    handle_step(machine)
}

fn handle_continue(machine: &mut MachineW) -> Result<(), AsmodeusError> {
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

fn handle_breakpoint(machine: &mut MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
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

fn handle_remove_breakpoint(machine: &mut MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
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

fn handle_list_breakpoints(machine: &MachineW) {
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

fn handle_memory_dump(machine: &MachineW, parts: &[&str]) -> Result<(), AsmodeusError> {
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

fn parse_address(addr_str: &str) -> Result<u16, AsmodeusError> {
    if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
        // hex
        u16::from_str_radix(&addr_str[2..], 16).map_err(|_| {
            AsmodeusError::UsageError(format!("Invalid hexadecimal address: {}", addr_str))
        })
    } else {
        // decimal
        addr_str.parse::<u16>().map_err(|_| {
            AsmodeusError::UsageError(format!("Invalid decimal address: {}", addr_str))
        })
    }
}

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
        Mode::Disassemble => run_mode_disassemble(&args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
