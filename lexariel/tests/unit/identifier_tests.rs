use lexariel::{tokenize, TokenKind};

#[test]
fn test_identifiers() {
    let tokens = tokenize("variable1 my_label COUNTER _start test123").unwrap();
    
    let expected_identifiers = vec!["variable1", "my_label", "COUNTER", "_start", "test123"];
    
    assert_eq!(tokens.len(), expected_identifiers.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_identifiers.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::Identifier);
        assert_eq!(tokens[i].value, *expected);
    }
}

#[test]
fn test_label_definitions() {
    let tokens = tokenize("start: loop: end_label:").unwrap();
    
    let expected_labels = vec!["start", "loop", "end_label"];
    
    assert_eq!(tokens.len(), expected_labels.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_labels.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::LabelDef);
        assert_eq!(tokens[i].value, *expected);
    }
}
