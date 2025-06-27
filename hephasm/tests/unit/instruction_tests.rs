use hephasm::assemble_source;

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
fn test_soz_instruction() {
    let machine_code = assemble_source("SOZ 100").unwrap();
    assert_eq!(machine_code.len(), 1);

    let expected = (0b10000u16 << 11) | 100;
    assert_eq!(machine_code[0], expected);
}
