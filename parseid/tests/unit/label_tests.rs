use parseid::{parse_source, ast::*};

#[test]
fn test_label_definition() {
    let program = parse_source("start:").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::LabelDefinition(label) = &program.elements[0] {
        assert_eq!(label.name, "start");
    } else {
        panic!("Expected label definition");
    }
}

#[test]
fn test_label_with_instruction() {
    let program = parse_source(r#"
        loop:
        DOD counter
        SOB loop
    "#).unwrap();
    
    assert_eq!(program.elements.len(), 3);
    
    // First element should be label
    if let ProgramElement::LabelDefinition(label) = &program.elements[0] {
        assert_eq!(label.name, "loop");
    } else {
        panic!("Expected label definition");
    }
    
    // Second element should be instruction
    if let ProgramElement::Instruction(inst) = &program.elements[1] {
        assert_eq!(inst.opcode, "DOD");
    } else {
        panic!("Expected instruction");
    }
}
