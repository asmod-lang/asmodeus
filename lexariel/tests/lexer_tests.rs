use lexariel::{tokenize, Lexer, Token, TokenKind, LexerError};

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
fn test_keywords() {
    let input = "DOD ODE ŁAD LAD POB SOB SOM SOZ STP DNS PZS SDP CZM MSK PWR WEJSCIE WYJSCIE";
    let tokens = tokenize(input).unwrap();
    
    let expected_keywords = vec![
        "DOD", "ODE", "ŁAD", "LAD", "POB", "SOB", "SOM", "SOZ", "STP", 
        "DNS", "PZS", "SDP", "CZM", "MSK", "PWR", "WEJSCIE", "WYJSCIE"
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
fn test_label_definitions() {
    let tokens = tokenize("start: loop: end_label:").unwrap();
    
    let expected_labels = vec!["start", "loop", "end_label"];
    
    assert_eq!(tokens.len(), expected_labels.len() + 1); // +1 for EOF
    
    for (i, expected) in expected_labels.iter().enumerate() {
        assert_eq!(tokens[i].kind, TokenKind::LabelDef);
        assert_eq!(tokens[i].value, *expected);
    }
}

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

#[test]
fn test_mixed_tokens() {
    let input = r#"
        start:          ; Label definition
            DOD 100     ; Add value at address 100
            SOB loop    ; Jump to loop
            RST 42      ; Reserve and initialize
        loop:
            POB 0xFF    ; Load hex value
            STP         ; Stop execution
    "#;
    
    let tokens = tokenize(input).unwrap();
    
    let expected = vec![
        (TokenKind::LabelDef, "start"),
        (TokenKind::Keyword, "DOD"),
        (TokenKind::Number, "100"),
        (TokenKind::Keyword, "SOB"),
        (TokenKind::Identifier, "loop"),
        (TokenKind::Directive, "RST"),
        (TokenKind::Number, "42"),
        (TokenKind::LabelDef, "loop"),
        (TokenKind::Keyword, "POB"),
        (TokenKind::Number, "0xFF"),
        (TokenKind::Keyword, "STP"),
        (TokenKind::Eof, ""),
    ];
    
    assert_eq!(tokens.len(), expected.len());
    
    for (i, (expected_kind, expected_value)) in expected.iter().enumerate() {
        assert_eq!(tokens[i].kind, *expected_kind, "Token {} kind mismatch", i);
        assert_eq!(tokens[i].value, *expected_value, "Token {} value mismatch", i);
    }
}

#[test]
fn test_line_and_column_tracking() {
    let input = "DOD\n  POB 123\n    STP";
    let tokens = tokenize(input).unwrap();
    
    // DOD at line 1, column 1
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);
    
    // POB at line 2, column 3 (after 2 spaces)
    assert_eq!(tokens[1].line, 2);
    assert_eq!(tokens[1].column, 3);
    
    // 123 at line 2, column 7
    assert_eq!(tokens[2].line, 2);
    assert_eq!(tokens[2].column, 7);
    
    // STP at line 3, column 5 (after 4 spaces)
    assert_eq!(tokens[3].line, 3);
    assert_eq!(tokens[3].column, 5);
}

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

#[test]
fn test_complex_assembly_program() {
    let input = r#"
        ; Machine W Assembly Program
        // Another comment style
        
        MAKRO add_two_numbers    ; Macro definition
            DOD first_num        ; Add first number
            DOD second_num       ; Add second number
        KONM
        
        start:                   ; Program entry point
            RST 0x10            ; Reserve with hex value
            RST 0b1010          ; Reserve with binary value  
            POB [100]           ; Load with addressing
            add_two_numbers     ; Call macro
            STP                 ; Stop program
            
        first_num: RST 25       ; Data definition
        second_num: RST -10     ; Negative data
    "#;
    
    let tokens = tokenize(input).unwrap();
    
    // should not have any invalid tokens and should properly categorize everything
    for token in &tokens {
        match token.kind {
            TokenKind::Keyword | TokenKind::Directive | TokenKind::Identifier | 
            TokenKind::Number | TokenKind::LabelDef | TokenKind::Punctuation | TokenKind::Eof => {
                // all valid
            }
        }
    }
    
    let keyword_count = tokens.iter().filter(|t| t.kind == TokenKind::Keyword).count();
    let directive_count = tokens.iter().filter(|t| t.kind == TokenKind::Directive).count();
    let label_count = tokens.iter().filter(|t| t.kind == TokenKind::LabelDef).count();
    
    assert!(keyword_count > 0, "Should have keywords");
    assert!(directive_count > 0, "Should have directives");
    assert!(label_count > 0, "Should have label definitions");
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

#[test]
fn test_polish_characters_in_keywords() {
    let tokens = tokenize("ŁAD").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[0].value, "ŁAD");
}
