use std::path::PathBuf;
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;
use asmachina::MachineW;

fn get_example_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(filename);
    path
}

#[test]
fn test_example_hello() {
    let source = r#"
        start:
            POB message
            WYJSCIE
            STP
        message: RST 42
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[42]);
}

#[test]
fn test_example_add() {
    let source = r#"
        start:
            POB num1
            DOD num2
            WYJSCIE
            STP
        num1: RST 25
        num2: RST 17
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[42]);
}
