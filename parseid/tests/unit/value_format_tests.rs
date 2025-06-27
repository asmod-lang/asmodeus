use parseid::{parse_source, ast::*};

#[test]
fn test_hexadecimal_and_binary_values() {
    let program = parse_source(r#"
        DOD 0xFF
        POB 0b1010
        ŁAD #0x1234
    "#).unwrap();
    
    assert_eq!(program.elements.len(), 3);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.value, "0xFF");
    }
    
    if let ProgramElement::Instruction(inst) = &program.elements[1] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.value, "0b1010");
    }
    
    if let ProgramElement::Instruction(inst) = &program.elements[2] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::Immediate);
        assert_eq!(operand.value, "0x1234");
    }
}
