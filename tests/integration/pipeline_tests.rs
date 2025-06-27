//! lex → parse → assemble → run

use asmachina::MachineW;
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;

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
