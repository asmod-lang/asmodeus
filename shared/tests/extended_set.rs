use asmodeus_shared::opcodes::Opcode;

#[test]
fn test_extended_opcode_identification() {
    assert!(Opcode::MNO.is_extended());
    assert!(Opcode::DZI.is_extended());
    assert!(Opcode::MOD.is_extended());
    
    // should not be extended
    assert!(!Opcode::DOD.is_extended());
    assert!(!Opcode::ODE.is_extended());
    assert!(!Opcode::POB.is_extended());
    assert!(!Opcode::STP.is_extended());
}

#[test]
fn test_extended_opcode_values() {
    assert_eq!(Opcode::MNO as u8, 0b10001);
    assert_eq!(Opcode::DZI as u8, 0b10010);
    assert_eq!(Opcode::MOD as u8, 0b10011);
}

#[test]
fn test_extended_opcode_from_u8() {
    assert_eq!(Opcode::from_u8(0b10001), Some(Opcode::MNO));
    assert_eq!(Opcode::from_u8(0b10010), Some(Opcode::DZI));
    assert_eq!(Opcode::from_u8(0b10011), Some(Opcode::MOD));
    
    // invalid extended opcode
    assert_eq!(Opcode::from_u8(0b10100), None);
}

#[test]
fn test_extended_opcodes_require_operand() {
    assert!(Opcode::MNO.requires_operand());
    assert!(Opcode::DZI.requires_operand());
    assert!(Opcode::MOD.requires_operand());
}

#[test]
fn test_all_extended_opcodes_enumeration() {
    let extended_opcodes: Vec<Opcode> = [
        Opcode::MNO,
        Opcode::DZI, 
        Opcode::MOD,
    ].to_vec();
    
    for opcode in extended_opcodes {
        assert!(opcode.is_extended(), "Opcode {:?} should be marked as extended", opcode);
        assert!(opcode.requires_operand(), "Extended opcode {:?} should require operand", opcode);
    }
}
