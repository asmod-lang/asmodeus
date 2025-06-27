use hephasm::assemble_source;

#[test]
fn test_macro_definition_and_call() {
    let machine_code = assemble_source(r#"
        MAKRO add_two value1 value2
            DOD value1
            DOD value2
        KONM
        
        add_two 10 20
        STP
    "#).unwrap();
    
    assert_eq!(machine_code.len(), 3);
    
    // DOD 10
    let expected_dod1 = (0b00001u16 << 11) | 10;
    assert_eq!(machine_code[0], expected_dod1);
    
    // DOD 20
    let expected_dod2 = (0b00001u16 << 11) | 20;
    assert_eq!(machine_code[1], expected_dod2);
    
    // STP
    let expected_stp = 0b00111u16 << 11;
    assert_eq!(machine_code[2], expected_stp);
}
