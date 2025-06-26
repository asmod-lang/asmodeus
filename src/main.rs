//! Asmodeus - Machine W Emulator and Assembler

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use asmodeus_core::MachineW;
use asmodeus_lexer::tokenize;
use asmodeus_parser::parse;
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

#[derive(Debug, PartialEq)]
enum Mode {
    Assemble,
    Disassemble,
    Run,
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
        return Ok(Args {
            mode: Mode::Help,
            input_file: None,
            output_file: None,
            verbose: false,
            debug: false,
        });
    }

    let mut mode = Mode::Run;
    let mut input_file = None;
    let mut output_file = None;
    let mut verbose = false;
    let mut debug = false;
    
    let mut i = 1;
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
    println!("Usage: asmodeus [OPTIONS] <INPUT_FILE>");
    println!();
    println!("OPTIONS:");
    println!("  -r, --run         Run the assembly program (default)");
    println!("  -a, --assemble    Assemble to binary without running");
    println!("  -d, --disassemble Disassemble binary file");
    println!("  -o, --output      Specify output file");
    println!("  -v, --verbose     Verbose output");
    println!("  --debug           Debug output");
    println!("  -h, --help        Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  asmodeus program.asmod         # Run assembly program");
    println!("  asmodeus -a program.asmod      # Assemble to binary");
    println!("  asmodeus -d program.bin        # Disassemble binary");
    println!("  asmodeus -o output.bin -a prog.asmod  # Assemble with output file");
    println!();
    println!("SUPPORTED FILE EXTENSIONS:");
    println!("  .asmod    Asmodeus assembly source files");
    println!("  .asm      Alternative assembly source files"); 
    println!("  .bin      Binary machine code files");
}

fn validate_file_extension(path: &str, mode: Mode) -> Result<(), AsmodeusError> {
    let path = Path::new(path);
    let extension = path.extension().and_then(|ext| ext.to_str());
    
    match (mode, extension) {
        (Mode::Run | Mode::Assemble, Some("asmod")) => Ok(()),
        (Mode::Run | Mode::Assemble, Some("asm")) => Ok(()),
        (Mode::Disassemble, Some("bin")) => Ok(()),
        (Mode::Run | Mode::Assemble, Some(ext)) => {
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
                "File has no extension. Please use .asmod for source files or .bin for binary files.".to_string()
            ))
        }
        _ => Ok(()),
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
        println!("Tokens: {:?}", tokens);
    }
    
    if args.verbose {
        println!("Parsing tokens to AST...");
    }
    
    let ast = parse(tokens)?;
    
    if args.debug {
        println!("AST: {:?}", ast);
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
    println!("  AK (Accumulator): {} (0x{:04X})", machine.ak, machine.ak);
    println!("  L  (PC):          {} (0x{:04X})", machine.l, machine.l);
    println!("  WS (Stack):       {} (0x{:04X})", machine.ws, machine.ws);
    println!("  Running:          {}", machine.is_running);
    println!("  Interrupts:       {}", machine.interrupts_enabled);
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
        Mode::Disassemble => run_mode_disassemble(&args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
