use dismael::disassemble;

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
