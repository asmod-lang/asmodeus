use hephasm::assemble_source;

#[test]
fn test_complex_program() {
    let machine_code = assemble_source(r#"
        ; Simple program with data and labels
        start:
            POB value       ; Load value
            ŁAD result      ; Store in result
            STP             ; Stop
            
        value: RST 42
        result: RPA
    "#).unwrap();
    
    // assembles without errors?
    assert!(!machine_code.is_empty());
    assert_eq!(machine_code.len(), 5);
    
    // POB value (address 3)
    let expected_pob = (0b00100u16 << 11) | 3;
    assert_eq!(machine_code[0], expected_pob);
    
    // ŁAD result (address 4)
    let expected_lad = (0b00011u16 << 11) | 4;
    assert_eq!(machine_code[1], expected_lad);
    
    // STP
    let expected_stp = 0b00111u16 << 11;
    assert_eq!(machine_code[2], expected_stp);
    
    // value: RST 42
    assert_eq!(machine_code[3], 42);
    
    // result: RPA
    assert_eq!(machine_code[4], 0);
}

#[test]
fn test_program_with_comments_and_whitespace() {
    let machine_code = assemble_source(r#"
        ; This is a test program
        start:          ; Main entry point
            DOD #10     ; Add immediate value 10
            STP         ; Stop execution
            
        ; Data section
        data: RST 42    ; Test data
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // DOD #10
    let expected_dod = (0b00001u16 << 11) | 10;
    assert_eq!(machine_code[0], expected_dod);
    
    // STP
    let expected_stp = 0b00111u16 << 11;
    assert_eq!(machine_code[1], expected_stp);
    
    // data: RST 42
    assert_eq!(machine_code[2], 42);
}

#[test]
fn test_empty_program() {
    let machine_code = assemble_source("").unwrap();
    assert_eq!(machine_code.len(), 0);
}
