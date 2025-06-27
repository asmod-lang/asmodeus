use hephasm::assemble_source;

#[test]
fn test_error_undefined_symbol() {
    let result = assemble_source("SOB undefined_label");
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    let error_string = error.to_string();
    println!("Actual error: {}", error_string); // Debug print: can be removed later (TODO)
    assert!(error_string.contains("Undefined symbol") || error_string.contains("undefined_label"));
}

#[test]
fn test_error_duplicate_label() {
    let result = assemble_source(r#"
        label:
        DOD 100
        label:
        STP
    "#);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_opcode() {
    let result = assemble_source("INVALID 100");
    assert!(result.is_err());
}

#[test]
fn test_error_address_out_of_bounds() {
    let result = assemble_source("DOD 9999");  // too large for 11-bit address
    assert!(result.is_err());
}

#[test]
fn test_error_macro_not_found() {
    let result = assemble_source("undefined_macro 1 2 3");
    assert!(result.is_err());
}

#[test]
fn test_error_macro_parameter_mismatch() {
    let result = assemble_source(r#"
        MAKRO test_macro param1 param2
            DOD param1
        KONM
        
        test_macro 1  ; Missing second parameter
    "#);
    assert!(result.is_err());
}
