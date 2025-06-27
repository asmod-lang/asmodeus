use parseid::{parse_source, ParserError};

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
