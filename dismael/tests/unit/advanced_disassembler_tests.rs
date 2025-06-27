use dismael::AdvancedDisassembler;

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
