use hephasm::{assemble_source, assemble_program_extended, AssemblerError};

#[test]
fn test_extended_instructions_without_flag() {
    // fail without --extended flag
    let result = assemble_source("MNO 100");
    assert!(result.is_err());
    
    if let Err(AssemblerError::ExtendedInstructionNotEnabled { instruction, line }) = result {
        assert_eq!(instruction, "MNO");
        assert_eq!(line, 1);
    } else {
        panic!("Expected ExtendedInstructionNotEnabled error");
    }
}

#[test]
fn test_extended_instructions_with_flag() {
    // work with --extended flag
    let machine_code = assemble_program_extended("MNO 100").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    // MNO = 0b10001, direct addressing mode = 0b000, argument = 100
    let expected = (0b10001u16 << 11) | (0b000u16 << 8) | 100;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_all_extended_opcodes() {
    let machine_code = assemble_program_extended(r#"
        MNO 10
        DZI 20  
        MOD 30
        STP
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 4);
    
    let mno_opcode = (machine_code[0] >> 11) & 0b11111;
    let dzi_opcode = (machine_code[1] >> 11) & 0b11111;
    let mod_opcode = (machine_code[2] >> 11) & 0b11111;
    let stp_opcode = (machine_code[3] >> 11) & 0b11111;
    
    assert_eq!(mno_opcode, 0b10001); // MNO
    assert_eq!(dzi_opcode, 0b10010); // DZI
    assert_eq!(mod_opcode, 0b10011); // MOD
    assert_eq!(stp_opcode, 0b00111); // STP
}

#[test]
fn test_extended_instructions_immediate_addressing() {
    let machine_code = assemble_program_extended(r#"
        MNO #42
        DZI #7
        MOD #5
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // immediate addressing mode (001) and arguments
    for (i, expected_arg) in [42, 7, 5].iter().enumerate() {
        let instruction = machine_code[i];
        let addressing_mode = (instruction >> 8) & 0b111;
        let argument = instruction & 0xFF;
        
        assert_eq!(addressing_mode, 0b001); // immediate
        assert_eq!(argument, *expected_arg);
    }
}

#[test]
fn test_extended_instructions_all_addressing_modes() {
    let machine_code = assemble_program_extended(r#"
        ; Data section
        value: RST 42
        ptr: RST 200
        
        ; Instructions with different addressing modes
        MNO value      ; direct
        DZI #10        ; immediate  
        MOD [ptr]      ; indirect
        STP
    "#).unwrap();
    
    assert!(machine_code.len() >= 5);
    
    let mno_mode = (machine_code[2] >> 8) & 0b111; // direct (after data)
    let dzi_mode = (machine_code[3] >> 8) & 0b111; // immediate
    let mod_mode = (machine_code[4] >> 8) & 0b111; // indirect
    
    assert_eq!(mno_mode, 0b000); // direct
    assert_eq!(dzi_mode, 0b001); // immediate
    assert_eq!(mod_mode, 0b010); // indirect
}

#[test]
fn test_mixed_standard_and_extended_instructions() {
    let machine_code = assemble_program_extended(r#"
        DOD 100    ; standard instruction
        MNO #5     ; extended instruction
        ODE 200    ; standard instruction  
        DZI #2     ; extended instruction
        STP        ; standard instruction
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 5);
    
    let opcodes: Vec<u16> = machine_code.iter()
        .map(|&instruction| (instruction >> 11) & 0b11111)
        .collect();
    
    assert_eq!(opcodes, vec![0b00001, 0b10001, 0b00010, 0b10010, 0b00111]);
    // DOD, MNO, ODE, DZI, STP
}

#[test]
fn test_extended_instruction_error_messages() {
    // DZI without extended flag
    let result = assemble_source("DZI 10");
    assert!(result.is_err());
    
    if let Err(AssemblerError::ExtendedInstructionNotEnabled { instruction, .. }) = result {
        assert_eq!(instruction, "DZI");
    }
    
    // MOD without extended flag
    let result = assemble_source("MOD 10");
    assert!(result.is_err());
    
    if let Err(AssemblerError::ExtendedInstructionNotEnabled { instruction, .. }) = result {
        assert_eq!(instruction, "MOD");
    }
}
