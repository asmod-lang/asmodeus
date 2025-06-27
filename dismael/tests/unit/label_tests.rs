use dismael::disassemble;

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
