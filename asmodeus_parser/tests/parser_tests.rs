use asmodeus_parser::{parse_source, ast::*, ParserError};
use lexariel::tokenize;

#[test]
fn test_empty_program() {
    let program = parse_source("").unwrap();
    assert_eq!(program.elements.len(), 0);
}

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

#[test]
fn test_error_unexpected_token() {
    let result = parse_source("DOD @");
    assert!(result.is_err());
    
    if let Err(ParserError::LexerError(_)) = result {
        // Expected lexer error for invalid character '@'
    } else {
        panic!("Expected lexer error");
    }
}

#[test]
fn test_error_missing_konm() {
    let result = parse_source(r#"
        MAKRO test_macro
            DOD 100
        ; Missing KONM
    "#);
    
    assert!(result.is_err());
    
    if let Err(ParserError::InvalidMacroDefinition { message, .. }) = result {
        assert!(message.contains("Missing KONM"));
    } else {
        panic!("Expected InvalidMacroDefinition error");
    }
}

#[test]
fn test_error_unexpected_eof() {
    let result = parse_source("DOD");
    assert!(result.is_err());
    
    match result {
        Err(ParserError::UnexpectedEof { expected }) => {
            assert!(expected.contains("operand"));
        }
        Err(other_error) => {
            panic!("Expected UnexpectedEof error, but got: {:?}", other_error);
        }
        Ok(_) => {
            panic!("Expected error but parsing succeeded");
        }
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
