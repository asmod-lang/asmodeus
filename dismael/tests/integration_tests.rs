//! Integration tests for the dismael disassembler module

use dismael::disassemble;
use hephasm::assemble_source;

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
