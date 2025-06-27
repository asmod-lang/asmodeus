use hephasm::assemble_source;

#[test]
fn test_hexadecimal_numbers() {
    let machine_code = assemble_source("DOD 0xFF").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    let expected = (0b00001u16 << 11) | 0xFF;
    assert_eq!(machine_code[0], expected);
}

#[test]
fn test_binary_numbers() {
    let machine_code = assemble_source("DOD 0b1010").unwrap();
    assert_eq!(machine_code.len(), 1);
    
    let expected = (0b00001u16 << 11) | 0b1010;
    assert_eq!(machine_code[0], expected);
}
