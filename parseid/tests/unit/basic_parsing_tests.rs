use parseid::{parse_source};

#[test]
fn test_empty_program() {
    let program = parse_source("").unwrap();
    assert_eq!(program.elements.len(), 0);
}
