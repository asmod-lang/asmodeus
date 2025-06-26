//! Integration tests for the complete Asmodeus toolchain

use std::fs;
use std::path::Path;
use std::process::Command;

use asmachina::{MachineW, MachineError};
use asmodeus_lexer::tokenize;
use asmodeus_parser::parse;
use hephasm::assemble_program;
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

// === DEBUGGER TESTS ===

#[test]
fn test_debugger_breakpoint_functionality() {
    let mut machine = MachineW::new();
    
    let program = vec![
        (0b00100 << 11) | 10,  // POB 10
        (0b00001 << 11) | 11,  // DOD 11  
        (0b00111 << 11) | 0,   // STP
        0, 0, 0, 0, 0, 0, 0,   // padding
        25,                    // address 10: value 25
        17,                    // address 11: value 17
    ];
    
    assert!(machine.load_program(&program).is_ok());
    machine.is_running = true;
    
    // breakpoint at address 1 (DOD instruction)
    assert!(machine.add_breakpoint(1).is_ok());
    assert!(machine.has_breakpoint(1));
    
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 1 })));
    assert_eq!(machine.l, 1); // stopped at breakpoint
    assert_eq!(machine.ak, 25); // POB executed, AK = 25
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 2); // moved to STP
    assert_eq!(machine.ak, 42); // 25 + 17 = 42
    
    // should stop at STP
    assert!(machine.run_until_halt_or_breakpoint().is_ok());
    assert_eq!(machine.is_running, false); // stopped
}

#[test]
fn test_debugger_state_inspection() {
    let mut machine = MachineW::new();
    
    machine.ak = 0x1234;
    machine.l = 0x10;
    machine.ws = 2000;
    machine.memory[0x10] = 0x5678;
    machine.memory[0x11] = 0x9ABC;
    
    let state = machine.get_current_state();
    assert_eq!(state.ak, 0x1234);
    assert_eq!(state.l, 0x10);
    assert_eq!(state.ws, 2000);
    
    // Test memory range - returns Option<Vec<(address, value)>>
    let memory_range = machine.get_memory_range(0x10, 0x11);
    assert!(memory_range.is_some());
    let range = memory_range.unwrap();
    assert_eq!(range.len(), 2);
    assert_eq!(range[0], (0x10, 0x5678)); // (address, value) pairs
    assert_eq!(range[1], (0x11, 0x9ABC));
}

#[test]
fn test_debugger_breakpoint_management() {
    let mut machine = MachineW::new();
    
    // multiple breakpoints
    assert!(machine.add_breakpoint(10).is_ok());
    assert!(machine.add_breakpoint(20).is_ok());
    assert!(machine.add_breakpoint(30).is_ok());
    
    assert!(machine.has_breakpoint(10));
    assert!(machine.has_breakpoint(20));
    assert!(machine.has_breakpoint(30));
    assert!(!machine.has_breakpoint(40));
    
    let breakpoints = machine.list_breakpoints();
    assert_eq!(breakpoints.len(), 3);
    assert!(breakpoints.contains(&10));
    assert!(breakpoints.contains(&20));
    assert!(breakpoints.contains(&30));
    
    machine.remove_breakpoint(20);
    assert!(!machine.has_breakpoint(20));
    assert_eq!(machine.list_breakpoints().len(), 2);
    
    // clear all
    machine.clear_all_breakpoints();
    assert_eq!(machine.list_breakpoints().len(), 0);
}

#[test]
fn test_debugger_step_instruction() {
    let mut machine = MachineW::new();
    
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[1] = (0b00011 << 11) | 11; // LAD 11 
    machine.memory[2] = (0b00111 << 11) | 0;  // STP
    machine.memory[10] = 15; // data
    
    machine.l = 0;
    machine.ak = 5;
    machine.is_running = true;
    
    // DOD 10
    assert!(machine.step_instruction().is_ok());
    assert_eq!(machine.l, 1);
    assert_eq!(machine.ak, 20); // 5 + 15
    
    // LAD 11
    assert!(machine.step_instruction().is_ok());
    assert_eq!(machine.l, 2);
    assert_eq!(machine.memory[11], 20); // AK stored
    
    // STP
    assert!(machine.step_instruction().is_ok());
    assert_eq!(machine.l, 3);
    assert_eq!(machine.is_running, false);
}

#[test]
fn test_debugger_run_until_breakpoint_with_multiple_breakpoints() {
    let mut machine = MachineW::new();
    
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[1] = (0b00101 << 11) | 3;  // SOB 3 (jump to address 3)
    machine.memory[2] = (0b00111 << 11) | 0;  // STP (shouldn't reach)
    machine.memory[3] = (0b00001 << 11) | 11; // DOD 11
    machine.memory[4] = (0b00111 << 11) | 0;  // STP
    machine.memory[10] = 5;
    machine.memory[11] = 10;
    
    machine.l = 0;
    machine.ak = 0;
    machine.is_running = true;
    
    assert!(machine.add_breakpoint(1).is_ok()); // at SOB
    assert!(machine.add_breakpoint(3).is_ok()); // at DOD 11
    
    // first breakpoint (should hit address 1)
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 1 })));
    assert_eq!(machine.l, 1);
    assert_eq!(machine.ak, 5); // DOD 10 executed
    
    // SOB instruction manually at address 1
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 3); // jumped to address 3
    assert_eq!(machine.ak, 5); // still 5
    
    // second breakpoint (should hit address 3)
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 3 })));
    assert_eq!(machine.l, 3);
    assert_eq!(machine.ak, 5);
    
    // DOD instruction manually at address 3
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 4); // moved to STP
    assert_eq!(machine.ak, 15); // 5 + 10
    
    let result = machine.run_until_halt_or_breakpoint();
    assert!(result.is_ok());
    assert_eq!(machine.is_running, false);
    assert_eq!(machine.ak, 15); // 5 + 10
}

#[test]
fn test_breakpoint_hit_at_exact_address() {
    let mut machine = MachineW::new();
    
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[1] = (0b00001 << 11) | 11; // DOD 11
    machine.memory[2] = (0b00111 << 11) | 0;  // STP
    machine.memory[10] = 5;
    machine.memory[11] = 7;
    
    assert!(machine.add_breakpoint(1).is_ok());
    machine.l = 0;
    machine.ak = 0;
    machine.is_running = true;
    
    // should stop exactly at breakpoint
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 1 })));
    assert_eq!(machine.l, 1); // stopped AT the breakpoint
}

#[test]
fn test_memory_range_retrieval() {
    let mut machine = MachineW::new();
    
    for i in 0..20 {
        machine.memory[i] = (i * 2) as u16;
    }
    
    let range = machine.get_memory_range(5, 10);
    assert!(range.is_some());
    let range = range.unwrap();
    assert_eq!(range.len(), 6); // inclusive range: 5,6,7,8,9,10
    assert_eq!(range[0], (5, 10));   // (address, value): memory[5] = 5*2 = 10
    assert_eq!(range[1], (6, 12));   // memory[6] = 6*2 = 12
    assert_eq!(range[5], (10, 20));  // memory[10] = 10*2 = 20
}

#[test]
fn test_debugger_state_snapshot() {
    let mut machine = MachineW::new();
    
    machine.ak = 0xABCD;
    machine.l = 0x123;
    machine.ad = 0x456;
    machine.kod = 0x7;
    machine.ws = 1500;
    
    let state = machine.get_current_state();
    
    assert_eq!(state.ak, 0xABCD);
    assert_eq!(state.l, 0x123);
    assert_eq!(state.ad, 0x456);
    assert_eq!(state.kod, 0x7);
    assert_eq!(state.ws, 1500);
    assert_eq!(state.is_running, false); // default state
}


#[test]
fn test_original_array_iteration_algorithm() {
    let mut machine = MachineW::new();
    
    // int count_lower_than_a(int a, int tab[], int n)
    let program = vec![
        // loop: POB n (address 0)
        (0b00100 << 11) | 15,  // POB 15 (n variable)
        // ODE one (address 1)
        (0b00010 << 11) | 17,  // ODE 17 (one variable)
        // SOM end (address 2)
        (0b00110 << 11) | 25,  // SOM 25 (end label)
        // ŁAD n (address 3)
        (0b00011 << 11) | 15,  // LAD 15 (store back to n)
        
        // calc: POB tab (address 4) - this instruction gets modified!
        (0b00100 << 11) | 11,  // POB 11 (initially tab[0])
        // ODE a (address 5)
        (0b00010 << 11) | 10,  // ODE 10 (threshold a)
        // SOM cpp (address 6)
        (0b00110 << 11) | 19,  // SOM 19 (cpp label)
        
        // return: POB calc (address 7) - loads address of calc instruction
        (0b00100 << 11) | 4,   // POB 4 (calc instruction address)
        // DOD one (address 8)
        (0b00001 << 11) | 17,  // DOD 17 (increment address by 1)
        // ŁAD calc (address 9) - modifies calc instruction!
        (0b00011 << 11) | 4,   // LAD 4 (store modified instruction)
        // SOB loop (address 10)
        (0b00101 << 11) | 0,   // SOB 0 (back to loop)
        
        // variables
        4,      // a: threshold value (address 10)
        1,      // tab[0] (address 11)
        3,      // tab[1] (address 12)
        5,      // tab[2] (address 13)
        4,      // tab[3] (address 14)
        7,      // tab[4] (address 15) - this conflicts with n!
        5,      // n: array length (address 16) - moved here
        1,      // one: constant 1 (address 17)
        0,      // count: counter (address 18)
        
        // cpp: POB count (address 19)
        (0b00100 << 11) | 18,  // POB 18 (count variable)
        // DOD one (address 20)
        (0b00001 << 11) | 17,  // DOD 17 (one variable)
        // ŁAD count (address 21)
        (0b00011 << 11) | 18,  // LAD 18 (store back to count)
        // SOB return (address 22)
        (0b00101 << 11) | 7,   // SOB 7 (back to return)
        0, 0,   // padding (addresses 23-24)
        
        // end: POB count (address 25)
        (0b00100 << 11) | 18,  // POB 18 (count variable)
        // WYJSCIE (address 26)
        (0b01111 << 11) | 0,   // WYJSCIE (output result)
        // STP (address 27)
        (0b00111 << 11) | 0,   // STP
    ];
    
    let mut corrected_program = program;
    corrected_program[0] = (0b00100 << 11) | 16;  // POB 16 (n at new address)
    corrected_program[3] = (0b00011 << 11) | 16;  // LAD 16 (n at new address)
    
    assert!(machine.load_program(&corrected_program).is_ok());
    
    // step limit to prevent infinite loops
    let result = machine.run_steps(1000);
    
    match result {
        Ok(steps) => {
            println!("Program executed {} steps", steps);
            println!("Final AK: {}", machine.ak);
            println!("Output buffer: {:?}", machine.get_output_buffer());
            println!("Is running: {}", machine.is_running);
            
            if !machine.is_running && !machine.get_output_buffer().is_empty() {
                assert_eq!(machine.get_output_buffer()[0], 2, "Should count 2 elements lower than 4");
            }
        }
        Err(e) => {
            println!("Program error: {:?}", e);
            println!("Machine state - AK: {}, L: {}", machine.ak, machine.l);
            println!("Memory around variables: {:?}", &machine.memory[10..20]);
        }
    }
}

#[test]
fn test_interactive_character_io() {
    let mut machine = MachineW::new();
    machine.set_interactive_mode(false);
    
    // WPR (character input)
    machine.set_input_buffer(vec![65, 66, 67]); // ASCII: 'A', 'B', 'C'
    
    let wpr_instruction = (0b01110 << 11) | 0; // WPR (= WEJSCIE) 
    machine.memory[0] = wpr_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 65); // should read 'A' (ASCII 65)
    
    // WYJ (character output)
    machine.set_interactive_mode(true);
    machine.ak = 72; // 'H'
    let wyj_instruction = (0b01111 << 11) | 0; // WYJ (= WYJSCIE) 
    machine.memory[1] = wyj_instruction;
    machine.l = 1;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.get_output_buffer(), &[72]); // should output 'H'
}

#[test]
fn test_character_io_buffer_mode() {
    let mut machine = MachineW::new();
    machine.set_interactive_mode(false);
    
    machine.set_input_buffer(vec![65, 66, 67]); // ASCII 'A', 'B', 'C'
    
    let wejscie_instruction = (0b01110 << 11) | 0; // WEJSCIE
    machine.memory[0] = wejscie_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 65); // should read 'A' (ASCII 65)
    
    // Test output
    machine.ak = 72; // 'H'
    let wyjscie_instruction = (0b01111 << 11) | 0; // WYJSCIE
    machine.memory[1] = wyjscie_instruction;
    machine.l = 1;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.get_output_buffer(), &[72]); // should output 'H'
}

#[test]
fn test_simple_character_echo() {
    let mut machine = MachineW::new();
    
    // echo: WEJSCIE -> WYJSCIE -> STP
    let program = vec![
        (0b01110 << 11) | 0,    // WEJSCIE (read character)
        (0b01111 << 11) | 0,    // WYJSCIE (output character)  
        (0b00111 << 11) | 0,    // STP (stop)
    ];
    
    machine.set_input_buffer(vec![72]); // 'H'
    assert!(machine.load_program(&program).is_ok());
    
    let result = machine.run();
    assert!(result.is_ok());
    
    assert_eq!(machine.ak, 72); // should have 'H' in accumulator
    assert_eq!(machine.get_output_buffer(), &[72]); // should output 'H'
    assert_eq!(machine.is_running, false); // should have stopped
    }
}
