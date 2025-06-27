use lexariel::{tokenize, TokenKind};

#[test]
fn test_decimal_numbers() {
    let tokens = tokenize("0 123 -456 999").unwrap();
    
    let expected_numbers = vec!["0", "123", "-456", "999"];
    
    assert_eq!(tokens.len(), expected_numbers.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_numbers.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::Number);
        assert_eq!(tokens[i].value, *expected);
    }
}

#[test]
fn test_hexadecimal_numbers() {
    let tokens = tokenize("0x0 0xABCD 0xff 0X123").unwrap();
    
    let expected_numbers = vec!["0x0", "0xABCD", "0xff", "0X123"];
    
    assert_eq!(tokens.len(), expected_numbers.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_numbers.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::Number);
        assert_eq!(tokens[i].value, *expected);
    }
}

#[test]
fn test_binary_numbers() {
    let tokens = tokenize("0b0 0b1010 0b11111111 0B0101").unwrap();
    
    let expected_numbers = vec!["0b0", "0b1010", "0b11111111", "0B0101"];
    
    assert_eq!(tokens.len(), expected_numbers.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_numbers.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::Number);
        assert_eq!(tokens[i].value, *expected);
    }
}

#[test]
fn test_negative_numbers_vs_minus() {
    let tokens = tokenize("-123 - 456").unwrap();
    
    assert_eq!(tokens.len(), 4); // -123, -, 456, EOF
    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].value, "-123");
    assert_eq!(tokens[1].kind, TokenKind::Punctuation);
    assert_eq!(tokens[1].value, "-");
    assert_eq!(tokens[2].kind, TokenKind::Number);
    assert_eq!(tokens[2].value, "456");
}
