use lexariel::{tokenize, TokenKind};

#[test]
fn test_empty_input() {
    let tokens = tokenize("").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}

#[test]
fn test_whitespace_handling() {
    let tokens = tokenize("   \t\n\r  ").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}

#[test]
fn test_edge_case_empty_lines() {
    let input = "\n\n\nDOD\n\n\nSTP\n\n";
    let tokens = tokenize(input).unwrap();
    
    assert_eq!(tokens.len(), 3); // DOD, STP, EOF
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[0].value, "DOD");
    assert_eq!(tokens[1].kind, TokenKind::Keyword);
    assert_eq!(tokens[1].value, "STP");
    assert_eq!(tokens[2].kind, TokenKind::Eof);
}
