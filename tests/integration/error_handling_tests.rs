use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;

#[test]
fn test_error_handling_lexer() {
    let source = "DOD @invalid";
    
    let result = tokenize(source);
    assert!(result.is_err());
}

#[test]
fn test_error_handling_parser() {
    let source = "DOD"; // missing operand (<value>)
    
    let tokens = tokenize(source).unwrap();
    let result = parse(tokens);
    assert!(result.is_err());
}

#[test]
fn test_error_handling_assembler() {
    let source = "SOB undefined_label";
    
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let result = assemble_program(&ast);
    assert!(result.is_err());
}
