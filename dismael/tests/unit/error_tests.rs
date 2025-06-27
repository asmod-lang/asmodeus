use dismael::{disassemble, DisassemblerError};

#[test]
fn test_empty_machine_code() {
    let result = disassemble(&[]);
    assert!(matches!(result, Err(DisassemblerError::EmptyCode)));
}
