use lexariel::{tokenize, LexerError};

#[test]
fn test_invalid_hex_number() {
    let result = tokenize("0x");
    assert!(result.is_err());
    
    if let Err(LexerError::InvalidNumberFormat { line, column, value }) = result {
        assert_eq!(line, 1);
        assert_eq!(column, 1);
        assert_eq!(value, "0x");
    } else {
        panic!("Expected InvalidNumberFormat error");
    }
}

#[test]
fn test_invalid_binary_number() {
    let result = tokenize("0b");
    assert!(result.is_err());
    
    if let Err(LexerError::InvalidNumberFormat { line, column, value }) = result {
        assert_eq!(line, 1);
        assert_eq!(column, 1);
        assert_eq!(value, "0b");
    } else {
        panic!("Expected InvalidNumberFormat error");
    }
}

#[test]
fn test_invalid_character() {
    let result = tokenize("DOD @");
    assert!(result.is_err());
    
    if let Err(LexerError::InvalidCharacter { line, column, character }) = result {
        assert_eq!(line, 1);
        assert_eq!(column, 5);
        assert_eq!(character, '@');
    } else {
        panic!("Expected InvalidCharacter error");
    }
}
