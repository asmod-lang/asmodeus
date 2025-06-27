use dismael::{disassemble, disassemble_to_string};

#[test]
fn test_simple_instruction_disassembly() {
    // DOD 100 instruction
    let machine_code = vec![(0b00001 << 11) | 100];
    let result = disassemble(&machine_code).unwrap();
    
    assert_eq!(result.len(), 1); // just the instruction
    assert!(result[0].contains("DOD"));
    assert!(result[0].contains("100"));
}

#[test]
fn test_instruction_without_operand() {
    // STP instruction
    let machine_code = vec![0b00111 << 11];
    let result = disassemble(&machine_code).unwrap();
    
    assert_eq!(result.len(), 1);
    assert!(result[0].contains("STP"));
    assert!(!result[0].contains("0")); // should not have operand
}

#[test]
fn test_all_opcodes() {
    let instructions = vec![
        (0b00001 << 11) | 1,   // DOD
        (0b00010 << 11) | 2,   // ODE
        (0b00011 << 11) | 3,   // ŁAD
        (0b00100 << 11) | 4,   // POB
        (0b00101 << 11) | 5,   // SOB
        (0b00110 << 11) | 6,   // SOM
        (0b00111 << 11) | 0,   // STP
        (0b01000 << 11) | 0,   // DNS
        (0b01001 << 11) | 0,   // PZS
        (0b01010 << 11) | 0,   // SDP
        (0b01011 << 11) | 0,   // CZM
        (0b01100 << 11) | 11,  // MSK
        (0b01101 << 11) | 0,   // PWR
        (0b01110 << 11) | 13,  // WEJSCIE
        (0b01111 << 11) | 14,  // WYJSCIE
    ];

    let expected_mnemonics = vec![
        "DOD", "ODE", "ŁAD", "POB", "SOB", "SOM", "STP",
        "DNS", "PZS", "SDP", "CZM", "MSK", "PWR", "WEJSCIE", "WYJSCIE"
    ];

    let result = disassemble(&instructions).unwrap();
    
    for (i, expected) in expected_mnemonics.iter().enumerate() {
        let instruction_line = result.iter()
            .find(|line| line.contains(expected))
            .expect(&format!("Could not find instruction {}", expected));
        assert!(instruction_line.contains(expected));
    }
}

#[test]
fn test_disassemble_to_string() {
    let machine_code = vec![
        (0b00001 << 11) | 100,  // DOD 100
        (0b00111 << 11) | 0,    // STP
    ];

    let result = disassemble_to_string(&machine_code).unwrap();
    
    assert!(result.contains("DOD"));
    assert!(result.contains("STP"));
    assert!(result.contains("100"));
}
