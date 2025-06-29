use lexariel::{tokenize, TokenKind};

#[test]
fn test_keywords() {
    let input = "DOD ODE ŁAD LAD POB SOB SOM SOZ STP DNS PZS SDP CZM MSK PWR WEJSCIE WYJSCIE MNO DZI MOD";
    let tokens = tokenize(input).unwrap();
    
    let expected_keywords = vec![
        "DOD", "ODE", "ŁAD", "LAD", "POB", "SOB", "SOM", "SOZ", "STP", 
        "DNS", "PZS", "SDP", "CZM", "MSK", "PWR", "WEJSCIE", "WYJSCIE",
        "MNO", "DZI", "MOD"
    ];
    
    assert_eq!(tokens.len(), expected_keywords.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_keywords.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::Keyword);
        assert_eq!(tokens[i].value, *expected);
    }

    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_keywords_case_insensitive() {
    let tokens = tokenize("dod OdE łaD").unwrap();
    assert_eq!(tokens.len(), 4); // 3 keywords + EOF
    
    for i in 0..3 {
        assert_eq!(tokens[i].kind, TokenKind::Keyword);
    }
    
    assert_eq!(tokens[0].value, "dod");
    assert_eq!(tokens[1].value, "OdE");
    assert_eq!(tokens[2].value, "łaD");
}

#[test]
fn test_directives() {
    let input = "RST RPA MAKRO KONM NAZWA_LOKALNA";
    let tokens = tokenize(input).unwrap();
    
    let expected_directives = vec!["RST", "RPA", "MAKRO", "KONM", "NAZWA_LOKALNA"];
    
    assert_eq!(tokens.len(), expected_directives.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_directives.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::Directive);
        assert_eq!(tokens[i].value, *expected);
    }
}

#[test]
fn test_polish_characters_in_keywords() {
    let tokens = tokenize("ŁAD").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[0].value, "ŁAD");
}

#[test]
fn test_extended_keywords() {
    let input = "MNO DZI MOD";
    let tokens = tokenize(input).unwrap();
    
    assert_eq!(tokens.len(), 4); // 3 keywords + EOF
    
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[0].value, "MNO");
    
    assert_eq!(tokens[1].kind, TokenKind::Keyword);
    assert_eq!(tokens[1].value, "DZI");
    
    assert_eq!(tokens[2].kind, TokenKind::Keyword);
    assert_eq!(tokens[2].value, "MOD");
    
    assert_eq!(tokens[3].kind, TokenKind::Eof);
}
