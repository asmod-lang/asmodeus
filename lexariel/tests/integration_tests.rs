use lexariel::{tokenize, TokenKind};

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
