use parseid::{parse_source, ast::*};

#[test]
fn test_complex_program() {
    let program = parse_source(r#"
        ; Complex assembly program
        MAKRO multiply_by_two value
            DOD value
            DOD value
        KONM
        
        start:
            RST 100
            POB #42
            ŁAD data
            multiply_by_two data
            SOB end
            
        data: RST 25
        end: STP
    "#).unwrap();
    
    let mut macro_count = 0;
    let mut label_count = 0;
    let mut instruction_count = 0;
    let mut directive_count = 0;
    
    for element in &program.elements {
        match element {
            ProgramElement::MacroDefinition(_) => macro_count += 1,
            ProgramElement::LabelDefinition(_) => label_count += 1,
            ProgramElement::Instruction(_) => instruction_count += 1,
            ProgramElement::Directive(_) => directive_count += 1,
            ProgramElement::MacroCall(_) => {}
        }
    }
    
    assert!(macro_count > 0, "Should have macro definitions");
    assert!(label_count > 0, "Should have label definitions");
    assert!(instruction_count > 0, "Should have instructions");
    assert!(directive_count > 0, "Should have directives");
}
