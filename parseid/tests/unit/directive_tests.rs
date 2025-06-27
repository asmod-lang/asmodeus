use parseid::{parse_source, ast::*};

#[test]
fn test_directive() {
    let program = parse_source("RST 42").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Directive(dir) = &program.elements[0] {
        assert_eq!(dir.name, "RST");
        assert_eq!(dir.arguments, vec!["42"]);
    } else {
        panic!("Expected directive");
    }
}

#[test]
fn test_directive_without_arguments() {
    let program = parse_source("RPA").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Directive(dir) = &program.elements[0] {
        assert_eq!(dir.name, "RPA");
        assert_eq!(dir.arguments.len(), 0);
    } else {
        panic!("Expected directive");
    }
}
