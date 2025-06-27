use lexariel::{tokenize, TokenKind};

#[test]
fn test_punctuation() {
    let tokens = tokenize("[]{}(),.#+*/=-<>!&|^~").unwrap();
    
    let expected_chars: Vec<char> = "[]{}(),.#+*/=-<>!&|^~".chars().collect();
    
    assert_eq!(tokens.len(), expected_chars.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_chars.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::Punctuation);
        assert_eq!(tokens[i].value, expected.to_string());
    }
}

#[test]
fn test_minus_as_punctuation() {
    let tokens = tokenize("DOD - ODE").unwrap();
    
    assert_eq!(tokens.len(), 4); // DOD, -, ODE, EOF
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[0].value, "DOD");
    assert_eq!(tokens[1].kind, TokenKind::Punctuation);
    assert_eq!(tokens[1].value, "-");
    assert_eq!(tokens[2].kind, TokenKind::Keyword);
    assert_eq!(tokens[2].value, "ODE");
}
