//! Integration tests for the complete Asmodeus toolchain

use std::fs;
use std::path::Path;
use std::process::Command;

use asmodeus_core::MachineW;
use asmodeus_lexer::tokenize;
use asmodeus_parser::parse;
use asmodeus_assembler::assemble_program;
use asmodeus_disassembler::disassemble;

#[test]
fn test_complete_pipeline_simple() {
    let source = r#"
        start:
            POB data
            WYJSCIE
            STP
        data: RST 42
    "#;

    // lexing
    let tokens = tokenize(source).expect("Lexing failed");
    assert!(!tokens.is_empty());

    // parsing
    let ast = parse(tokens).expect("Parsing failed");
    assert!(!ast.elements.is_empty());

    // assembling
    let machine_code = assemble_program(&ast).expect("Assembly failed");
    assert!(!machine_code.is_empty());

    // emulating
    let mut machine = MachineW::new();
    machine.load_program(&machine_code).expect("Loading failed");
    machine.run().expect("Execution failed");

    // results
    assert_eq!(machine.is_running, false);
    assert_eq!(machine.get_output_buffer(), &[42]);
}

#[test]
fn test_complete_pipeline_with_arithmetic() {
    let source = r#"
        start:
            POB num1
            DOD num2
            WYJSCIE
            STP
        num1: RST 25
        num2: RST 17
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[42]); // 25 + 17
    assert_eq!(machine.ak, 42);
}

#[test]
fn test_complete_pipeline_with_jumps() {
    let source = r#"
        start:
            POB counter
            DOD one
            ŁAD counter
            POB counter      ; Reload updated counter
            SOM start        ; Jump back to start if counter < 0
        end:
            POB counter
            WYJSCIE
            STP
        counter: RST -3
        one: RST 1
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    // should count from -3 - 0
    assert_eq!(machine.get_output_buffer(), &[0]);
}

#[test]
fn test_complete_pipeline_with_stack() {
    let source = r#"
        start:
            POB val1
            SDP         ; Push val1
            POB val2
            SDP         ; Push val2
            PZS         ; Pop val2
            PZS         ; Pop val1
            DOD val2    ; Add val2 (still in memory)
            WYJSCIE
            STP
        val1: RST 10
        val2: RST 30
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[40]); // 10 + 30
}

#[test]
fn test_complete_pipeline_with_macros() {
    let source = r#"
        MAKRO add_and_output val1 val2
            POB val1
            DOD val2
            WYJSCIE
        KONM

        start:
            add_and_output data1 data2
            STP

        data1: RST 15
        data2: RST 25
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[40]); // 15 + 25
}

#[test]
fn test_error_handling_lexer() {
    let source = "DOD @invalid";
    
    let result = tokenize(source);
    assert!(result.is_err());
}

#[test]
fn test_error_handling_parser() {
    let source = "DOD"; // missing operand (<value>)
    
    let tokens = tokenize(source).unwrap();
    let result = parse(tokens);
    assert!(result.is_err());
}

#[test]
fn test_error_handling_assembler() {
    let source = "SOB undefined_label";
    
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let result = assemble_program(&ast);
    assert!(result.is_err());
}

#[test]
fn test_roundtrip_disassembly() {
    let source = r#"
        start:
            POB data
            DOD value
            WYJSCIE
            SOB loop
        loop:
            STP
        data: RST 10
        value: RST 5
    "#;

    // assemble
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    // disassemble
    let disassembled = disassemble(&machine_code).unwrap();
    let disasm_text = disassembled.join("\n");

    // key elements that were preserved
    assert!(disasm_text.contains("POB"));
    assert!(disasm_text.contains("DOD"));
    assert!(disasm_text.contains("WYJSCIE"));
    assert!(disasm_text.contains("SOB"));
    assert!(disasm_text.contains("STP"));
    
    // should have labels for jump targets
    assert!(disasm_text.contains("L_"));
    
    // should preserve data values
    assert!(disasm_text.contains("10"));
    assert!(disasm_text.contains("5"));
}

#[test]
fn test_roundtrip_complex_program() {
    let source = r#"
        ; Complex test program
        MAKRO multiply_by_two value
            POB value
            DOD value
        KONM

        start:
            multiply_by_two number
            WYJSCIE
            SOM negative_test
            STP

        negative_test:
            POB negative
            WYJSCIE
            STP

        number: RST 21
        negative: RST -10
    "#;

    // full pipeline test
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[42]); // 21 * 2

    // disassembly test
    let disassembled = disassemble(&machine_code).unwrap();
    let disasm_text = disassembled.join("\n");
    
    assert!(disasm_text.contains("POB"));
    assert!(disasm_text.contains("DOD"));
    assert!(disasm_text.contains("WYJSCIE"));
    assert!(disasm_text.contains("SOM"));
    assert!(disasm_text.contains("STP"));
}

#[test]
fn test_all_instructions_integration() {
    let source = r#"
        start:
            ; Test arithmetic
            POB val1        ; Load 10
            DOD val2        ; Add 5 -> 15
            ODE val3        ; Sub 3 -> 12
            ŁAD result      ; Store result
            
            ; Test stack
            SDP             ; Push 12
            POB val4        ; Load 8
            PZS             ; Pop 12
            DOD val4        ; Add 8 -> 20
            
            ; Test output
            WYJSCIE         ; Output 20
            
            ; Test jump
            SOB end
            
        end:
            STP

        val1: RST 10
        val2: RST 5
        val3: RST 3
        val4: RST 8
        result: RPA
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[20]);
    assert_eq!(machine.ak, 20);
    assert_eq!(machine.is_running, false);
    
    // were the results stores? they should be
    assert_eq!(machine.read_memory(machine_code.len() as u16 - 1).unwrap(), 12);
}

#[test]
fn test_interrupt_handling_integration() {
    let source = r#"
        start:
            POB data
            SDP             ; Push data to stack
            DNS             ; Disable interrupts
            POB data
            WYJSCIE
            STP

        data: RST 123
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[123]);
    assert_eq!(machine.interrupts_enabled, false); // should be disabled
}

#[test]
fn test_memory_bounds_integration() {
    let source = r#"
        POB 0
        STP
    "#;
    
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    
    // should execute without errors (accessing address 0)
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
fn test_large_program_integration() {
    let mut source = String::from("start:\n");
    
    // 100 additions
    for i in 0..100 {
        source.push_str(&format!("    DOD data_{}\n", i));
    }
    
    source.push_str("    WYJSCIE\n");
    source.push_str("    STP\n\n");
    
    for i in 0..100 {
        source.push_str(&format!("data_{}: RST {}\n", i, i + 1));
    }

    let tokens = tokenize(&source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    // 1+2+...+100 = 5050
    assert_eq!(machine.get_output_buffer(), &[5050]);
}

#[cfg(test)]
mod example_files_tests {
    use super::*;
    use std::path::PathBuf;

    fn get_example_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("examples");
        path.push(filename);
        path
    }

    #[test]
    fn test_example_hello() {
        let source = r#"
            start:
                POB message
                WYJSCIE
                STP
            message: RST 42
        "#;

        let tokens = tokenize(source).unwrap();
        let ast = parse(tokens).unwrap();
        let machine_code = assemble_program(&ast).unwrap();

        let mut machine = MachineW::new();
        machine.load_program(&machine_code).unwrap();
        machine.run().unwrap();

        assert_eq!(machine.get_output_buffer(), &[42]);
    }

    #[test]
    fn test_example_add() {
        let source = r#"
            start:
                POB num1
                DOD num2
                WYJSCIE
                STP
            num1: RST 25
            num2: RST 17
        "#;

        let tokens = tokenize(source).unwrap();
        let ast = parse(tokens).unwrap();
        let machine_code = assemble_program(&ast).unwrap();

        let mut machine = MachineW::new();
        machine.load_program(&machine_code).unwrap();
        machine.run().unwrap();

        assert_eq!(machine.get_output_buffer(), &[42]);
    }
}


#[cfg(test)]
mod cli_tests {
    use std::process::Command;
    use std::fs;
    use std::path::Path;
    use super::*;

    #[test]
    fn test_cli_hello_program() {
        let output = Command::new("cargo")
            .args(&["run", "--", "examples/hello.asmod"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command failed with status: {}", output.status);
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Program execution completed successfully"));
        assert!(stdout.contains("AK: 002A (42)"));
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
        assert!(stdout.contains("AK: 002A (42)"));
    }

    #[test]
    fn test_cli_countdown_program() {
        let output = Command::new("cargo")
            .args(&["run", "--", "examples/countdown.asmod"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Program execution completed successfully"));
        // 3, 2, 1, 0
        assert!(stdout.contains("[0] 3"));
        assert!(stdout.contains("[1] 2"));
        assert!(stdout.contains("[2] 1"));
        assert!(stdout.contains("[3] 0"));
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
        assert!(stdout.contains("Asmodeus - Machine W Emulator and Assembler"));
        assert!(stdout.contains("Usage:"));
        assert!(stdout.contains("OPTIONS:"));
        assert!(stdout.contains("EXAMPLES:"));
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
        assert!(stdout.contains("Creating Machine W emulator"));
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
}
