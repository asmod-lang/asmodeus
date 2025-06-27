use parseid::{parse_source, ast::*};

#[test]
fn test_macro_definition() {
    let program = parse_source(r#"
        MAKRO add_numbers param1 param2
            DOD param1
            DOD param2
        KONM
    "#).unwrap();
    
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::MacroDefinition(macro_def) = &program.elements[0] {
        assert_eq!(macro_def.name, "add_numbers");
        assert_eq!(macro_def.parameters, vec!["param1", "param2"]);
        assert_eq!(macro_def.body.len(), 2);
    } else {
        panic!("Expected macro definition");
    }
}

#[test]
fn test_macro_call() {
    let program = parse_source("add_numbers 10 20").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::MacroCall(macro_call) = &program.elements[0] {
        assert_eq!(macro_call.name, "add_numbers");
        assert_eq!(macro_call.arguments, vec!["10", "20"]);
    } else {
        panic!("Expected macro call");
    }
}
