use dismael::{disassemble, disassemble_to_string, Disassembler, AdvancedDisassembler, DisassemblerError};
use hephasm::assemble_source;

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
fn test_jump_target_labels() {
    let machine_code = vec![
        (0b00101 << 11) | 2,   // SOB 2 (jump to address 2)
        (0b00001 << 11) | 0,   // DOD 0
        (0b00111 << 11) | 0,   // STP (target of jump)
    ];

    let result = disassemble(&machine_code).unwrap();
    
    // should have a label for address 2
    let disasm_text = result.join("\n");
    assert!(disasm_text.contains("L_0002:"));
    assert!(disasm_text.contains("SOB L_0002"));
}

#[test]
fn test_conditional_jump_labels() {
    let machine_code = vec![
        (0b00110 << 11) | 3,   // SOM 3 (conditional jump to address 3)
        (0b00001 << 11) | 0,   // DOD 0
        (0b00001 << 11) | 1,   // DOD 1
        (0b00111 << 11) | 0,   // STP (target of jump)
    ];

    let result = disassemble(&machine_code).unwrap();
    
    let disasm_text = result.join("\n");
    assert!(disasm_text.contains("L_0003:"));
    assert!(disasm_text.contains("SOM L_0003"));
}

#[test]
fn test_data_recognition() {
    let machine_code = vec![
        (0b00100 << 11) | 2,   // POB 2 (load from address 2)
        (0b00111 << 11) | 0,   // STP
        42,                    // Data at address 2
    ];

    let result = disassemble(&machine_code).unwrap();
    
    let disasm_text = result.join("\n");
    // should be recognized as data and get a label
    assert!(disasm_text.contains("DATA_0002:"));
    assert!(disasm_text.contains("POB DATA_0002"));
}

#[test]
fn test_complex_program_with_labels() {
    let machine_code = vec![
        (0b00100 << 11) | 5,   // POB 5 (load data)
        (0b00110 << 11) | 4,   // SOM 4 (conditional jump)
        (0b00001 << 11) | 6,   // DOD 6 (add data)
        (0b00101 << 11) | 0,   // SOB 0 (loop back)
        (0b00111 << 11) | 0,   // STP (exit point)
        100,                   // Data at address 5
        50,                    // Data at address 6
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    // should have labels for jump targets
    assert!(disasm_text.contains("L_0004:")); // SOM target
    assert!(disasm_text.contains("L_0000:")); // SOB target (loop)
    
    // should have data labels
    assert!(disasm_text.contains("DATA_0005:")); // first data
    assert!(disasm_text.contains("DATA_0006:")); // second data
    
    // should use labels in instructions
    assert!(disasm_text.contains("SOM L_0004"));
    assert!(disasm_text.contains("SOB L_0000"));
    assert!(disasm_text.contains("POB DATA_0005"));
    assert!(disasm_text.contains("DOD DATA_0006"));
}

#[test]
fn test_empty_machine_code() {
    let result = disassemble(&[]);
    assert!(matches!(result, Err(DisassemblerError::EmptyCode)));
}

#[test]
fn test_invalid_opcode() {
    // invalid opcode (all 1s in opcode field)
    let machine_code = vec![0b11111 << 11];
    let result = disassemble(&machine_code);
    
    // should treat as data or return error
    // for now, it treats unknown opcodes as RST data
    assert!(result.is_ok());
    let disasm = result.unwrap();
    let disasm_text = disasm.join("\n");
    assert!(disasm_text.contains("RST"));
}

#[test]
fn test_roundtrip_simple_program() {
    let source = r#"
        start:
            DOD data
            STP
        data: RST 42
    "#;

    let machine_code = assemble_source(source).unwrap();
    let disassembled = disassemble(&machine_code).unwrap();
    let disasm_text = disassembled.join("\n");
    
    // key elements are preserved?
    assert!(disasm_text.contains("DOD"));
    assert!(disasm_text.contains("STP"));
    assert!(disasm_text.contains("42")); // data value should be preserved
}

#[test]
fn test_roundtrip_with_jumps() {
    let source = r#"
        start:
            SOB end
            DOD 100
        end:
            STP
    "#;

    let machine_code = assemble_source(source).unwrap();
    let disassembled = disassemble(&machine_code).unwrap();
    let disasm_text = disassembled.join("\n");
    
    // should have jump instruction and label
    assert!(disasm_text.contains("SOB"));
    assert!(disasm_text.contains("STP"));
    // should have generated a label for the jump target
    assert!(disasm_text.contains("L_"));
}

#[test]
fn test_address_comments() {
    let machine_code = vec![
        (0b00001 << 11) | 100,  // DOD 100
        (0b00111 << 11) | 0,    // STP
    ];

    let result = disassemble(&machine_code).unwrap();
    
    // should have 2 instructions
    assert_eq!(result.len(), 2);
    assert!(result[0].contains("DOD"));
    assert!(result[1].contains("STP"));
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

#[test]
fn test_stack_operations() {
    let machine_code = vec![
        (0b01010 << 11) | 0,    // SDP (push)
        (0b01001 << 11) | 0,    // PZS (pop)
        (0b00111 << 11) | 0,    // STP
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    assert!(disasm_text.contains("SDP"));
    assert!(disasm_text.contains("PZS"));
    assert!(disasm_text.contains("STP"));
}

#[test]
fn test_interrupt_operations() {
    let machine_code = vec![
        (0b01000 << 11) | 0,    // DNS
        (0b01011 << 11) | 0,    // CZM
        (0b01100 << 11) | 0xFF, // MSK 255
        (0b01101 << 11) | 0,    // PWR
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    assert!(disasm_text.contains("DNS"));
    assert!(disasm_text.contains("CZM"));
    assert!(disasm_text.contains("MSK"));
    assert!(disasm_text.contains("PWR"));
    assert!(disasm_text.contains("255")); // MSK operand
}

#[test]
fn test_io_operations() {
    let machine_code = vec![
        (0b01110 << 11) | 1,    // WEJSCIE 1
        (0b01111 << 11) | 2,    // WYJSCIE 2
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    assert!(disasm_text.contains("WEJSCIE"));
    assert!(disasm_text.contains("WYJSCIE"));
    assert!(disasm_text.contains("1"));
    assert!(disasm_text.contains("2"));
}

#[test]
fn test_advanced_disassembler() {
    let mut advanced = AdvancedDisassembler::new();
    
    let machine_code = vec![
        (0b00100 << 11) | 3,   // POB 3
        (0b00101 << 11) | 0,   // SOB 0 (infinite loop)
        (0b00111 << 11) | 0,   // STP (unreachable)
        42,                    // Data
    ];

    let result = advanced.disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    // should properly identify code vs data
    assert!(disasm_text.contains("POB"));
    assert!(disasm_text.contains("SOB"));
    // should be identified as data
    assert!(disasm_text.contains("42"));
}

#[test]
fn test_large_program() {
    // scalability
    let mut machine_code = Vec::new();
    
    for i in 0..100 {
        if i % 10 == 0 {
            // jumps every 10 instructions
            machine_code.push((0b00101 << 11) | ((i + 5) % 100));
        } else if i % 10 == 5 {
            // data
            machine_code.push(i * 2);
        } else {
            // instructions
            machine_code.push((0b00001 << 11) | ((i + 1) % 100));
        }
    }

    let result = disassemble(&machine_code);
    assert!(result.is_ok());
    
    let disasm = result.unwrap();
    assert!(disasm.len() > 100); // at least as many lines as instructions
}

#[test]
fn test_label_uniqueness() {
    // unique labels even with many jump targets
    let machine_code = vec![
        (0b00101 << 11) | 4,   // SOB 4
        (0b00101 << 11) | 5,   // SOB 5
        (0b00101 << 11) | 6,   // SOB 6
        (0b00101 << 11) | 7,   // SOB 7
        (0b00111 << 11) | 0,   // STP (address 4)
        (0b00111 << 11) | 0,   // STP (address 5)
        (0b00111 << 11) | 0,   // STP (address 6)
        (0b00111 << 11) | 0,   // STP (address 7)
    ];

    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    
    // unique labels for each target
    assert!(disasm_text.contains("L_0004:"));
    assert!(disasm_text.contains("L_0005:"));
    assert!(disasm_text.contains("L_0006:"));
    assert!(disasm_text.contains("L_0007:"));
    
    // use them in jumps
    assert!(disasm_text.contains("SOB L_0004"));
    assert!(disasm_text.contains("SOB L_0005"));
    assert!(disasm_text.contains("SOB L_0006"));
    assert!(disasm_text.contains("SOB L_0007"));
}

#[test]
fn test_soz_instruction() {
    let machine_code = vec![
        (0b10000 << 11) | 100,
    ];
    
    let result = disassemble(&machine_code).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].contains("SOZ"));
    assert!(result[0].contains("L_0064"));
}

#[test]
fn test_soz_jump_labels() {
    let machine_code = vec![
        (0b10000 << 11) | 3,   // SOZ 3 (conditional jump to address 3)
        (0b00001 << 11) | 0,   // DOD 0
        (0b00001 << 11) | 1,   // DOD 1
        (0b00111 << 11) | 0,   // STP (target of jump)
    ];
    let result = disassemble(&machine_code).unwrap();
    let disasm_text = result.join("\n");
    assert!(disasm_text.contains("L_0003:"));
    assert!(disasm_text.contains("SOZ L_0003"));
}
