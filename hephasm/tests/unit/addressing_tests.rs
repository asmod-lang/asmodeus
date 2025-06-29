use hephasm::assemble_source;

#[test]
fn test_immediate_addressing() {
    let machine_code = assemble_source("POB #42").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    // POB opcode = 0b00100, immediate addressing mode = 0b001, value 42
    let expected = (0b00100u16 << 11) | (0b001u16 << 8) | 42;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_register_addressing() {
    let machine_code = assemble_source("POB R0").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    // POB opcode = 0b00100, register addressing mode = 0b100, register 0
    let expected = (0b00100u16 << 11) | (0b100u16 << 8) | 0;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_indexed_addressing() {
    let machine_code = assemble_source(r#"
        array: RST 0
        POB array[i]
        i: RST 1
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // POB array[i] should use direct addressing mode with address of array (0)
    let expected = (0b00100u16 << 11) | (0b000u16 << 8) | 0;
    assert_eq!(machine_code[1], expected);
}

#[test]
fn test_relative_addressing() {
    let machine_code = assemble_source(r#"
        SOB +2
        DOD 100
        STP
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // SOB +2 from address 0 should jump to address 2, using relative addressing mode
    let expected_sob = (0b00101u16 << 11) | (0b111u16 << 8) | 2;
    assert_eq!(machine_code[0], expected_sob);
}

#[test]
fn test_negative_relative_addressing() {
    let machine_code = assemble_source(r#"
        start:
        DOD 100
        SOB -1
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 2);
    
    // SOB -1 from address 1 should jump to address 0, using relative addressing mode
    let expected_sob = (0b00101u16 << 11) | (0b111u16 << 8) | 0;
    assert_eq!(machine_code[1], expected_sob);
}

#[test]
fn test_mixed_addressing_modes() {
    let machine_code = assemble_source(r#"
        ; Data definitions first
        variable: RST 42
        ptr: RST 0
        
        ; Instructions
        DOD #100        ; immediate
        POB variable    ; direct (address 0)
        ŁAD ptr         ; direct (address 1)
        STP             ; stop
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 6);
    
    // variable: RST 42
    assert_eq!(machine_code[0], 42);
    
    // ptr: RST 0
    assert_eq!(machine_code[1], 0);
    
    // DOD #100 - immediate addressing
    let expected_dod = (0b00001u16 << 11) | (0b001u16 << 8) | 100;
    assert_eq!(machine_code[2], expected_dod);
    
    // POB variable (address 0) - direct addressing
    let expected_pob = (0b00100u16 << 11) | (0b000u16 << 8) | 0;
    assert_eq!(machine_code[3], expected_pob);
    
    // ŁAD ptr (address 1) - direct addressing
    let expected_lad = (0b00011u16 << 11) | (0b000u16 << 8) | 1;
    assert_eq!(machine_code[4], expected_lad);
    
    // STP - no operand, direct addressing with argument 0
    let expected_stp = (0b00111u16 << 11) | (0b000u16 << 8) | 0;
    assert_eq!(machine_code[5], expected_stp);
}
