use parseid::{parse_source, ast::*};

#[test]
fn test_immediate_addressing() {
    let program = parse_source("POB #42").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::Immediate);
        assert_eq!(operand.value, "42");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_indirect_addressing() {
    let program = parse_source("POB [100]").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::Indirect);
        assert_eq!(operand.value, "100");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_multiple_indirect_addressing() {
    let program = parse_source("POB [[100]]").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::MultipleIndirect);
        assert_eq!(operand.value, "100");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_register_addressing() {
    let program = parse_source("POB R0").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::Register);
        assert_eq!(operand.value, "R0");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_register_indirect_addressing() {
    let program = parse_source("POB [R1]").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::RegisterIndirect);
        assert_eq!(operand.value, "R1");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_base_register_addressing() {
    let program = parse_source("POB R2[10]").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        if let AddressingMode::BaseRegister { base, offset } = &operand.addressing_mode {
            assert_eq!(base, "R2");
            assert_eq!(offset, "10");
        } else {
            panic!("Expected base register addressing mode");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_indexed_addressing() {
    let program = parse_source("POB array[i]").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        if let AddressingMode::Indexed { address, index } = &operand.addressing_mode {
            assert_eq!(address, "array");
            assert_eq!(index, "i");
        } else {
            panic!("Expected indexed addressing mode");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_relative_addressing() {
    let program = parse_source("SOB +10").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::Relative);
        assert_eq!(operand.value, "+10");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_negative_relative_addressing() {
    let program = parse_source("SOB -5").unwrap();
    assert_eq!(program.elements.len(), 1);
    
    if let ProgramElement::Instruction(inst) = &program.elements[0] {
        let operand = inst.operand.as_ref().unwrap();
        assert_eq!(operand.addressing_mode, AddressingMode::Relative);
        assert_eq!(operand.value, "-5");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_multiple_addressing_modes() {
    let program = parse_source(r#"
        DOD #100        ; immediate
        ODE variable    ; direct
        POB [address]   ; indirect
        ŁAD [[ptr]]     ; multiple indirect
        SOB R0          ; register
        SOM [R1]        ; register indirect
        DOD R2[5]       ; base register
        ODE array[i]    ; indexed
        SOB +10         ; relative positive
        SOM -5          ; relative negative
    "#).unwrap();
    
    assert_eq!(program.elements.len(), 10);
    
    let expected_modes = vec![
        AddressingMode::Immediate,
        AddressingMode::Direct,
        AddressingMode::Indirect,
        AddressingMode::MultipleIndirect,
        AddressingMode::Register,
        AddressingMode::RegisterIndirect,
        AddressingMode::BaseRegister { base: "R2".to_string(), offset: "5".to_string() },
        AddressingMode::Indexed { address: "array".to_string(), index: "i".to_string() },
        AddressingMode::Relative,
        AddressingMode::Relative,
    ];
    
    for (i, element) in program.elements.iter().enumerate() {
        if let ProgramElement::Instruction(inst) = element {
            let operand = inst.operand.as_ref().unwrap();
            assert_eq!(operand.addressing_mode, expected_modes[i], 
                      "Addressing mode mismatch at instruction {}", i);
        }
    }
}
