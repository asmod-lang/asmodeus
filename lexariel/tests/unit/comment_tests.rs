use lexariel::{tokenize, TokenKind};

#[test]
fn test_semicolon_comments() {
    let tokens = tokenize("DOD ; this is a comment\nODE ; another comment").unwrap();
    
    assert_eq!(tokens.len(), 3); // DOD, ODE, EOF
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[0].value, "DOD");
    assert_eq!(tokens[1].kind, TokenKind::Keyword);
    assert_eq!(tokens[1].value, "ODE");
    assert_eq!(tokens[2].kind, TokenKind::Eof);
}

#[test]
fn test_double_slash_comments() {
    let tokens = tokenize("POB // this is a comment\nSOB // another comment").unwrap();
    
    assert_eq!(tokens.len(), 3); // POB, SOB, EOF
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[0].value, "POB");
    assert_eq!(tokens[1].kind, TokenKind::Keyword);
    assert_eq!(tokens[1].value, "SOB");
    assert_eq!(tokens[2].kind, TokenKind::Eof);
}
