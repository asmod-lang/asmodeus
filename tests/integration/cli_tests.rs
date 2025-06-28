use std::process::Command;
use std::fs;
use std::path::Path;
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;

#[test]
fn test_cli_hello_program() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/hello.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed with status: {}", output.status);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Program execution completed successfully"));
    assert!(stdout.contains("OUTPUT"));
    assert!(stdout.contains("42 (0x002A)"));
}

#[test]
fn test_cli_addition_program() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/addition.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Program execution completed successfully"));
    assert!(stdout.contains("OUTPUT"));
    assert!(stdout.contains("42 (0x002A)"));
}

#[test]
fn test_cli_assemble_mode() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-a", "examples/hello.asmod", "-o", "test_output.bin"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Assembly successful"));
    
    // output file should exist 
    assert!(Path::new("test_output.bin").exists());
    let _ = fs::remove_file("test_output.bin");
}

#[test]
fn test_cli_disassemble_mode() {
    let source = r#"
        POB 42
        WYJSCIE
        STP
    "#;
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();
    
    let bytes: Vec<u8> = machine_code.iter()
        .flat_map(|&word| word.to_le_bytes())
        .collect();
    fs::write("test_input.bin", bytes).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-d", "test_input.bin"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("POB"));
    assert!(stdout.contains("WYJSCIE"));
    assert!(stdout.contains("STP"));
    
    let _ = fs::remove_file("test_input.bin");
}

#[test]
fn test_cli_invalid_file_extension() {
    fs::write("test_wrong.txt", "POB 42\nSTP").unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "test_wrong.txt"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Expected .asmod or .asm file"));
    
    let _ = fs::remove_file("test_wrong.txt");
}

#[test]
fn test_cli_missing_file() {
    let output = Command::new("cargo")
        .args(&["run", "--", "nonexistent.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to read file"));
}

#[test]
fn test_cli_lexer_error() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/error_lexer.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Lexer Error"));
}

#[test]
fn test_cli_parser_error() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/error_parser.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Parser Error"));
}

#[test]
fn test_cli_assembler_error() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/error_assembler.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Assembler Error"));
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("Usage: asmod <COMMAND> [OPTIONS] <INPUT_FILE>"));
    assert!(stdout.contains("COMMANDS:"));
    assert!(stdout.contains("OPTIONS:"));
    assert!(stdout.contains("EXAMPLES:"));
    
    assert!(stdout.contains("run"));
    assert!(stdout.contains("debug"));
    assert!(stdout.contains("assemble"));
    assert!(stdout.contains("interactive"));
}

#[test]
fn test_cli_verbose_mode() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-v", "examples/hello.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Reading source file"));
    assert!(stdout.contains("Tokenizing source code"));
    assert!(stdout.contains("Parsing tokens to AST"));
    assert!(stdout.contains("Assembling AST to machine code"));
    assert!(stdout.contains("Creating Asmachina emulator"));
}

#[test]
fn test_cli_no_arguments() {
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No arguments provided"));
    assert!(stderr.contains("Use --help for usage information"));
}

#[test]
fn test_cli_error_exit_code() {
    let output = Command::new("cargo")
        .args(&["run", "--", "nonexistent.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}
