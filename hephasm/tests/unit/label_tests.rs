use hephasm::assemble_source;

#[test]
fn test_label_resolution() {
    let machine_code = assemble_source(r#"
        SOB end
        DOD 100
        end: STP
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // SOB should jump to address 2 (where end: is), direct addressing
    let expected_sob = (0b00101u16 << 11) | (0b000u16 << 8) | 2;
    assert_eq!(machine_code[0], expected_sob);
    
    // DOD 100, direct addressing
    let expected_dod = (0b00001u16 << 11) | (0b000u16 << 8) | 100;
    assert_eq!(machine_code[1], expected_dod);
    
    // STP, direct addressing with argument 0
    let expected_stp = (0b00111u16 << 11) | (0b000u16 << 8) | 0;
    assert_eq!(machine_code[2], expected_stp);
}

#[test]
fn test_soz_with_label() {
    let machine_code = assemble_source(r#"
        start:
            SOZ end
            DOD 100
        end:
            STP
    "#).unwrap();
    assert_eq!(machine_code.len(), 3);

    // SOZ should jump to address 2 (where end: is), direct addressing
    let expected_soz = (0b10000u16 << 11) | (0b000u16 << 8) | 2;
    assert_eq!(machine_code[0], expected_soz);
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
