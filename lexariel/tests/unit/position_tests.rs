use lexariel::tokenize;

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
