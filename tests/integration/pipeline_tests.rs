//! lex → parse → assemble → run
use asmachina::MachineW;
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;
use std::process::Command;
use std::fs;
use tempfile::TempDir;

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

#[test]
fn test_full_pipeline_extended_instructions() {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("pipeline_test.asmod");
    let binary_file = temp_dir.path().join("pipeline_test.bin");
    let disasm_file = temp_dir.path().join("pipeline_test_disasm.asmod");
    
    let original_program = r#"
        ; Pipeline test with extended instructions
        start:
            POB value1     ; load 12
            MNO value2     ; multiply by 5 = 60
            DZI #4         ; divide by 4 = 15
            MOD #8         ; modulo 8 = 7
            WYJSCIE        ; output 7
            STP
            
        value1: RST 12
        value2: RST 5
    "#;
    
    fs::write(&source_file, original_program).unwrap();
    
    // with extended flag
    let assemble_output = Command::new("cargo")
        .args(&["run", "--", "assemble", "--extended", source_file.to_str().unwrap(), "-o", binary_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute assembler");
    
    assert!(assemble_output.status.success(), "Assembly failed: {}", String::from_utf8_lossy(&assemble_output.stderr));
    assert!(binary_file.exists(), "Binary file not created");
    
    // use source
    let run_output = Command::new("cargo")
        .args(&["run", "--", "run", "--extended", source_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute run command");
    
    assert!(run_output.status.success(), "Run failed: {}", String::from_utf8_lossy(&run_output.stderr));
    
    let stdout = String::from_utf8_lossy(&run_output.stdout);
    assert!(stdout.contains("7"), "Expected output '7', got: {}", stdout);
    
    // use binary
    let disasm_output = Command::new("cargo")
        .args(&["run", "--", "disassemble", binary_file.to_str().unwrap(), "-o", disasm_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute disassembler");
    
    assert!(disasm_output.status.success(), "Disassembly failed: {}", String::from_utf8_lossy(&disasm_output.stderr));
    assert!(disasm_file.exists(), "Disassembly file not created");
    
    // verify
    let disassembled_content = fs::read_to_string(&disasm_file).unwrap();
    assert!(disassembled_content.contains("MNO"), "Disassembly should contain MNO");
    assert!(disassembled_content.contains("DZI"), "Disassembly should contain DZI");
    assert!(disassembled_content.contains("MOD"), "Disassembly should contain MOD");

    let has_immediate = disassembled_content.contains("#4") || disassembled_content.contains("#8") || 
                       disassembled_content.contains("4") || disassembled_content.contains("8");
    assert!(has_immediate, "Disassembly should contain immediate values");
}

#[test]
fn test_error_propagation_extended_mode() {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("error_test.asmod");
    
    // division by zero
    let program_with_error = r#"
        start:
            POB value      ; load 42
            DZI #0         ; divide by zero (should error)
            STP
            
        value: RST 42
    "#;
    
    fs::write(&source_file, program_with_error).unwrap();
    
    // should fail with division by zero during execution
    let run_output = Command::new("cargo")
        .args(&["run", "--", "run", "--extended", source_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute run command");
    
    assert!(!run_output.status.success(), "Run should fail due to division by zero");
    
    let stderr = String::from_utf8_lossy(&run_output.stderr);
    assert!(stderr.contains("DivisionByZero") || stderr.contains("Division by zero"), 
            "Error message should mention division by zero: {}", stderr);
}

#[test]
fn test_mixed_standard_extended_pipeline() {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("mixed_test.asmod");
    
    let mixed_program = r#"
        ; Mixed instruction test: (10 + 5) * 3 / 2 % 7 = 1
        start:
            POB value1     ; load 10 (standard)
            DOD value2     ; add 5 = 15 (standard)
            MNO #3         ; multiply by 3 = 45 (extended)
            DZI #2         ; divide by 2 = 22 (extended)
            MOD #7         ; modulo 7 = 1 (extended)
            WYJSCIE        ; output result (standard)
            STP            ; stop (standard)
            
        value1: RST 10
        value2: RST 5
    "#;
    
    fs::write(&source_file, mixed_program).unwrap();
    
    let run_output = Command::new("cargo")
        .args(&["run", "--", "run", "--extended", source_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute run command");
    
    assert!(run_output.status.success(), "Run failed: {}", String::from_utf8_lossy(&run_output.stderr));
    
    let stdout = String::from_utf8_lossy(&run_output.stdout);
    assert!(stdout.contains("1"), "Expected output '1', got: {}", stdout);
}
