use hephasm::assemble_source;

#[test]
fn test_rst_directive() {
    let machine_code = assemble_source("RST 42").unwrap();
    assert_eq!(machine_code.len(), 1);
    assert_eq!(machine_code[0], 42);
}

#[test]
fn test_rpa_directive() {
    let machine_code = assemble_source("RPA").unwrap();
    assert_eq!(machine_code.len(), 1);
    assert_eq!(machine_code[0], 0);
}
