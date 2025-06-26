use asmodeus_assembler::{assemble_source, assemble_program, Assembler, AssemblerError};
use asmodeus_parser::parse_source;

#[test]
fn test_simple_instruction() {
    let machine_code = assemble_source("DOD 100").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    // DOD = 0b00001, argument = 100
    // expected: (0b00001 << 11) | 100 = 0x0864
    let expected = (0b00001u16 << 11) | 100;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_instruction_without_operand() {
    let machine_code = assemble_source("STP").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    // STP opcode = 0b00111, no argument
    let expected = 0b00111u16 << 11;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_immediate_addressing() {
    let machine_code = assemble_source("POB #42").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    // POB opcode = 0b00100, immediate value 42
    let expected = (0b00100u16 << 11) | 42;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_hexadecimal_numbers() {
    let machine_code = assemble_source("DOD 0xFF").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    let expected = (0b00001u16 << 11) | 0xFF;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_binary_numbers() {
    let machine_code = assemble_source("DOD 0b1010").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    let expected = (0b00001u16 << 11) | 0b1010;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_label_resolution() {
    let machine_code = assemble_source(r#"
        SOB end
        DOD 100
        end: STP
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // SOB should jump to address 2 (where end: is)
    let expected_sob = (0b00101u16 << 11) | 2;
    assert_eq!(machine_code[0], expected_sob);
    
    // DOD 100
    let expected_dod = (0b00001u16 << 11) | 100;
    assert_eq!(machine_code[1], expected_dod);
    
    // STP
    let expected_stp = 0b00111u16 << 11;
    assert_eq!(machine_code[2], expected_stp);
}

#[test]
fn test_rst_directive() {
    let machine_code = assemble_source("RST 42").unwrap();
    assert_eq!(machine_code.len(), 1);
    assert_eq!(machine_code[0], 42);
}

#[test]
fn test_rpa_directive() {
    let machine_code = assemble_source("RPA").unwrap();
    assert_eq!(machine_code.len(), 1);
    assert_eq!(machine_code[0], 0);
}

#[test]
fn test_macro_definition_and_call() {
    let machine_code = assemble_source(r#"
        MAKRO add_two value1 value2
            DOD value1
            DOD value2
        KONM
        
        add_two 10 20
        STP
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // DOD 10
    let expected_dod1 = (0b00001u16 << 11) | 10;
    assert_eq!(machine_code[0], expected_dod1);
    
    // DOD 20
    let expected_dod2 = (0b00001u16 << 11) | 20;
    assert_eq!(machine_code[1], expected_dod2);
    
    // STP
    let expected_stp = 0b00111u16 << 11;
    assert_eq!(machine_code[2], expected_stp);
}

#[test]
fn test_all_opcodes() {
    let machine_code = assemble_source(r#"
        DOD 1
        ODE 2
        ŁAD 3
        POB 4
        SOB 5
        SOM 6
        STP
        DNS
        PZS
        SDP
        CZM
        MSK 11
        PWR
        WEJSCIE
        WYJSCIE
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 15);
    
    let expected_opcodes = vec![
        0b00001, 0b00010, 0b00011, 0b00100, 0b00101, 0b00110, 0b00111,
        0b01000, 0b01001, 0b01010, 0b01011, 0b01100, 0b01101, 0b01110, 0b01111
    ];
    
    for (i, expected_opcode) in expected_opcodes.iter().enumerate() {
        let instruction = machine_code[i];
        let actual_opcode = (instruction >> 11) & 0b11111;
        assert_eq!(actual_opcode, *expected_opcode as u16, "Opcode mismatch at instruction {}", i);
    }
}

#[test]
fn test_relative_addressing() {
    let machine_code = assemble_source(r#"
        SOB +2
        DOD 100
        STP
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // SOB +2 from address 0 should jump to address 2
    let expected_sob = (0b00101u16 << 11) | 2;
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
    
    // SOB -1 from address 1 should jump to address 0
    let expected_sob = (0b00101u16 << 11) | 0;
    assert_eq!(machine_code[1], expected_sob);
}

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
fn test_register_addressing() {
    let machine_code = assemble_source("POB R0").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    let expected = (0b00100u16 << 11) | 0;  // R0 = register 0
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
    
    // POB array[i] should use address of array (0)
    let expected = (0b00100u16 << 11) | 0;
    assert_eq!(machine_code[1], expected);
}

#[test]
fn test_error_undefined_symbol() {
    let result = assemble_source("SOB undefined_label");
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    let error_string = error.to_string();
    println!("Actual error: {}", error_string); // Debug print: can be removed later (TODO)
    assert!(error_string.contains("Undefined symbol") || error_string.contains("undefined_label"));
}

#[test]
fn test_error_duplicate_label() {
    let result = assemble_source(r#"
        label:
        DOD 100
        label:
        STP
    "#);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_opcode() {
    let result = assemble_source("INVALID 100");
    assert!(result.is_err());
}

#[test]
fn test_error_address_out_of_bounds() {
    let result = assemble_source("DOD 9999");  // too large for 11-bit address
    assert!(result.is_err());
}

#[test]
fn test_error_macro_not_found() {
    let result = assemble_source("undefined_macro 1 2 3");
    assert!(result.is_err());
}

#[test]
fn test_error_macro_parameter_mismatch() {
    let result = assemble_source(r#"
        MAKRO test_macro param1 param2
            DOD param1
        KONM
        
        test_macro 1  ; Missing second parameter
    "#);
    assert!(result.is_err());
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
    
    // DOD #100
    let expected_dod = (0b00001u16 << 11) | 100;
    assert_eq!(machine_code[2], expected_dod);
    
    // POB variable (address 0)
    let expected_pob = (0b00100u16 << 11) | 0;
    assert_eq!(machine_code[3], expected_pob);
    
    // ŁAD ptr (address 1)
    let expected_lad = (0b00011u16 << 11) | 1;
    assert_eq!(machine_code[4], expected_lad);
    
    // STP
    let expected_stp = 0b00111u16 << 11;
    assert_eq!(machine_code[5], expected_stp);
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

#[test]
fn test_program_with_only_labels() {
    let machine_code = assemble_source(r#"
        start:
        middle:
        end:
    "#).unwrap();
    assert_eq!(machine_code.len(), 0);
}
