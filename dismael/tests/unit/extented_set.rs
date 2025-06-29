use dismael::disassemble;

#[test]
fn test_disassemble_extended_instructions() {
    let machine_code = vec![
        (0b10001 << 11) | 100,  // MNO 100
        (0b10010 << 11) | 200,  // DZI 200  
        (0b10011 << 11) | 50,   // MOD 50
        (0b00111 << 11) | 0,    // STP
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    assert!(disasm_text.contains("MNO"));
    assert!(disasm_text.contains("DZI"));
    assert!(disasm_text.contains("MOD"));
    assert!(disasm_text.contains("100"));
    assert!(disasm_text.contains("200"));
    assert!(disasm_text.contains("50"));
}

#[test]
fn test_disassemble_extended_immediate_addressing() {
    let machine_code = vec![
        (0b10001 << 11) | (0b001 << 8) | 42,  // MNO #42
        (0b10010 << 11) | (0b001 << 8) | 7,   // DZI #7
        (0b10011 << 11) | (0b001 << 8) | 5,   // MOD #5
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    assert!(disasm_text.contains("MNO"));
    assert!(disasm_text.contains("#42"));
    assert!(disasm_text.contains("DZI"));
    assert!(disasm_text.contains("#7"));
    assert!(disasm_text.contains("MOD"));
    assert!(disasm_text.contains("#5"));
}

#[test]
fn test_disassemble_extended_indirect_addressing() {
    let machine_code = vec![
        (0b10001 << 11) | (0b010 << 8) | 100, // MNO [100]
        (0b10010 << 11) | (0b010 << 8) | 200, // DZI [200]
        (0b10011 << 11) | (0b010 << 8) | 50,  // MOD [50]
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    assert!(disasm_text.contains("MNO"));
    assert!(disasm_text.contains("[100]"));
    assert!(disasm_text.contains("DZI"));
    assert!(disasm_text.contains("[200]"));
    assert!(disasm_text.contains("MOD"));
    assert!(disasm_text.contains("[50]"));
}

#[test]
fn test_disassemble_mixed_instructions() {
    let machine_code = vec![
        (0b00001 << 11) | 10,                 // DOD 10
        (0b10001 << 11) | (0b001 << 8) | 5,  // MNO #5
        (0b00010 << 11) | 20,                 // ODE 20
        (0b10010 << 11) | (0b001 << 8) | 2,  // DZI #2
        (0b10011 << 11) | (0b001 << 8) | 7,  // MOD #7
        (0b00111 << 11) | 0,                  // STP
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    // Check all instructions are present
    assert!(disasm_text.contains("DOD"));
    assert!(disasm_text.contains("MNO"));
    assert!(disasm_text.contains("ODE"));
    assert!(disasm_text.contains("DZI"));
    assert!(disasm_text.contains("MOD"));
    assert!(disasm_text.contains("STP"));
    
    // Check addressing modes
    assert!(disasm_text.contains("#5"));
    assert!(disasm_text.contains("#2"));
    assert!(disasm_text.contains("#7"));
}

#[test]
fn test_roundtrip_extended_instructions() {
    use hephasm::assemble_program_extended;
    
    let source = r#"
        ; Extended arithmetic operations
        MNO #6     ; multiply by 6
        DZI #2     ; divide by 2  
        MOD #5     ; modulo 5
        STP        ; stop
    "#;
    
    // Assemble with extended flag
    let machine_code = assemble_program_extended(source).unwrap();
    
    // Disassemble back
    let disassembled = disassemble(&machine_code).unwrap();
    let disasm_text = disassembled.join("\n");
    
    // Check that all extended instructions are correctly roundtripped
    assert!(disasm_text.contains("MNO"));
    assert!(disasm_text.contains("#6"));
    assert!(disasm_text.contains("DZI"));
    assert!(disasm_text.contains("#2"));
    assert!(disasm_text.contains("MOD"));
    assert!(disasm_text.contains("#5"));
    assert!(disasm_text.contains("STP"));
}
