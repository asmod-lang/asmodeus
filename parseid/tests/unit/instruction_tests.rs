use parseid::{parse_source, ast::*};

#[test]
fn test_simple_instruction() {
    let program = parse_source("DOD 100").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        assert_eq!(inst.opcode, "DOD");
        assert!(inst.operand.is_some());
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::Direct);
        assert_eq!(operand.value, "100");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_instruction_without_operand() {
    let program = parse_source("STP").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        assert_eq!(inst.opcode, "STP");
        assert!(inst.operand.is_none());
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_all_machine_w_instructions() {
    let program = parse_source(r#"
        DOD 100
        ODE 200  
        ŁAD 300
        POB 400
        SOB label
        SOM label
        STP
        DNS
        PZS
        SDP
        CZM
        MSK 0xFF
        PWR
        WEJSCIE
        WYJSCIE
        
        label:
    "#).unwrap();
    
    let keywords = vec![
        "DOD", "ODE", "ŁAD", "POB", "SOB", "SOM", "STP", 
        "DNS", "PZS", "SDP", "CZM", "MSK", "PWR", "WEJSCIE", "WYJSCIE"
    ];
    
    let mut instruction_count = 0;
    for element in &program.elements {
        if let ProgramElement::Instruction(inst) = element {
            assert!(keywords.contains(&inst.opcode.as_str()));
            instruction_count += 1;
        }
    }
    
    assert_eq!(instruction_count, 15);
}
