use std::process::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;

#[test]
fn test_cli_hello_program() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/basic/hello.asmod"])
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
        .args(&["run", "--", "examples/arithmetic/dod.asmod"])
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
        .args(&["run", "--", "-a", "examples/basic/hello.asmod", "-o", "test_output.bin"])
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
    assert!(stderr.contains("Expected .asmod"));
    
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
        .args(&["run", "--", "examples/errors/error_lexer.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Lexer Error"));
}

#[test]
fn test_cli_parser_error() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/errors/error_parser.asmod"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Parser Error"));
}

#[test]
fn test_cli_assembler_error() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/errors/error_assembler.asmod"])
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
        .args(&["run", "--", "-v", "examples/basic/hello.asmod"])
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

#[test]
fn test_extended_flag_assembles_extended_instructions() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test_extended.asmod");
    let output_file = temp_dir.path().join("test_extended.bin");
    
    let program = r#"
        ; Extended instruction test program
        start:
            POB value1
            MNO value2    ; multiply
            DZI #2        ; divide by 2
            MOD #10       ; modulo 10
            WYJSCIE       ; output result
            STP
            
        value1: RST 6
        value2: RST 7
    "#;
    
    fs::write(&input_file, program).unwrap();
    
    // assembly with --extended flag
    let output = Command::new("cargo")
        .args(&["run", "--", "assemble", "--extended", input_file.to_str().unwrap(), "-o", output_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute assembler");
    
    assert!(output.status.success(), "Assembly failed: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output_file.exists(), "Output file was not created");
    
    let binary_data = fs::read(&output_file).unwrap();
    assert!(!binary_data.is_empty(), "Binary file is empty");
}

#[test]
fn test_extended_flag_short_form() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test_extended_short.asmod");
    let output_file = temp_dir.path().join("test_extended_short.bin");
    
    let program = "MNO #5\nSTP\n";
    fs::write(&input_file, program).unwrap();
    
    // assembly with -e flag
    let output = Command::new("cargo")
        .args(&["run", "--", "assemble", "-e", input_file.to_str().unwrap(), "-o", output_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute assembler");
    
    assert!(output.status.success(), "Assembly with -e flag failed: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output_file.exists(), "Output file was not created");
}

#[test]
fn test_extended_instructions_without_flag_fail() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test_no_extended.asmod");
    let output_file = temp_dir.path().join("test_no_extended.bin");
    
    let program = r#"
        MNO #5
        STP
    "#;
    
    fs::write(&input_file, program).unwrap();
    
    // assembly without --extended flag (should fail)
    let output = Command::new("cargo")
        .args(&["run", "--", "assemble", input_file.to_str().unwrap(), "-o", output_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute assembler");
    
    assert!(!output.status.success(), "Assembly should have failed without --extended flag");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ExtendedInstructionNotEnabled") || stderr.contains("MNO"), 
            "Error message should mention extended instruction: {}", stderr);
}

#[test]
fn test_extended_run_mode_execution() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test_extended_run.asmod");
    
    // 6 * 7 / 3 % 5 = 14 % 5 = 4
    let program = r#"
        start:
            POB value      ; load 6
            MNO #7         ; multiply by 7 (42)
            DZI #3         ; divide by 3 (14)
            MOD #5         ; modulo 5 (4)
            WYJSCIE        ; output result
            STP
            
        value: RST 6
    "#;
    
    fs::write(&input_file, program).unwrap();
    
    // --extended flag (.asmod directly)
    let output = Command::new("cargo")
        .args(&["run", "--", "run", "--extended", input_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute run command");
    
    assert!(output.status.success(), "Extended run failed: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("4"), "Expected output '4', got: {}", stdout);
}

#[test]
fn test_extended_disassemble_mode() {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("test_extended_disasm.asmod");
    let binary_file = temp_dir.path().join("test_extended_disasm.bin");
    
    let program = r#"
        MNO #10
        DZI #2
        MOD #7
        STP
    "#;
    
    fs::write(&source_file, program).unwrap();
    
    // assemble with extended flag
    let assemble_output = Command::new("cargo")
        .args(&["run", "--", "assemble", "--extended", source_file.to_str().unwrap(), "-o", binary_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute assembler");
    
    assert!(assemble_output.status.success());
    
    // disassemble (.bin)
    let disasm_output = Command::new("cargo")
        .args(&["run", "--", "disassemble", binary_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute disassembler");
    
    assert!(disasm_output.status.success(), "Disassembly failed: {}", String::from_utf8_lossy(&disasm_output.stderr));
    
    let disasm_text = String::from_utf8_lossy(&disasm_output.stdout);
    assert!(disasm_text.contains("MNO"), "Disassembly should contain MNO instruction");
    assert!(disasm_text.contains("DZI"), "Disassembly should contain DZI instruction");
    assert!(disasm_text.contains("MOD"), "Disassembly should contain MOD instruction");
    // Check for immediate values - może być problem z formatowaniem
    let has_immediate_10 = disasm_text.contains("#10") || disasm_text.contains("10");
    assert!(has_immediate_10, "Disassembly should contain immediate value (looking for #10 or 10): {}", disasm_text);
}

#[test]
fn test_extended_help_message() {
    let output = Command::new("cargo")
        .args(&["run", "--", "assemble", "--help"])
        .output()
        .expect("Failed to execute help command");
    
    assert!(output.status.success());
    
    let help_text = String::from_utf8_lossy(&output.stdout);
    assert!(help_text.contains("--extended") || help_text.contains("-e"), 
            "Help should mention --extended flag: {}", help_text);
}

#[test]
fn test_complex_extended_program() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("complex_extended.asmod");
    
    let program = r#"
        ; Calculate factorial using extended instructions
        ; Computes 5! = 120, then 120 % 7 = 1
        start:
            POB n          ; load 5
            ŁAD counter    ; counter = 5
            POB one        ; result = 1
            ŁAD result
            
        loop:
            POB counter    ; load counter
            SOZ done       ; if counter == 0, done
            
            POB result     ; load current result
            MNO counter    ; multiply by counter (extended instruction)
            ŁAD result     ; store result
            
            POB counter    ; load counter
            ODE one        ; decrement counter
            ŁAD counter    ; store counter
            
            SOB loop       ; continue loop
            
        done:
            POB result     ; load final result (120)
            MOD modval     ; result % 7 (extended instruction)
            WYJSCIE        ; output final result (1)
            STP
            
        n:       RST 5
        one:     RST 1
        counter: RPA
        result:  RPA 
        modval:  RST 7
    "#;
    
    fs::write(&input_file, program).unwrap();
    
    // assembly and run (.asmod)
    let output = Command::new("cargo")
        .args(&["run", "--", "run", "--extended", input_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute complex extended program");
    
    assert!(output.status.success(), "Complex extended program failed: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1"), "Expected output '1' (120 % 7), got: {}", stdout);
}
