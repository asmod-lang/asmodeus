use dismael::disassemble;

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
fn test_soz_instruction() {
    let machine_code = vec![
        (0b10000 << 11) | 100,
    ];
    
    let result = disassemble(&machine_code).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].contains("SOZ"));
    assert!(result[0].contains("L_0064"));
}
