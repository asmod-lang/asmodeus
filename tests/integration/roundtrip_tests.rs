use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;
use dismael::disassemble;

#[test]
fn test_roundtrip_disassembly() {
    let source = r#"
        start:
            POB data
            DOD value
            WYJSCIE
            SOB loop
        loop:
            STP
        data: RST 10
        value: RST 5
    "#;

    // assemble
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    // disassemble
    let disassembled = disassemble(&machine_code).unwrap();
    let disasm_text = disassembled.join("\n");

    // key elements that were preserved
    assert!(disasm_text.contains("POB"));
    assert!(disasm_text.contains("DOD"));
    assert!(disasm_text.contains("WYJSCIE"));
    assert!(disasm_text.contains("SOB"));
    assert!(disasm_text.contains("STP"));
    
    // should have labels for jump targets
    assert!(disasm_text.contains("L_"));
    
    // should preserve data values
    assert!(disasm_text.contains("10"));
    assert!(disasm_text.contains("5"));
}

#[test]
fn test_roundtrip_complex_program() {
    let source = r#"
        ; Complex test program
        MAKRO multiply_by_two value
            POB value
            DOD value
        KONM

        start:
            multiply_by_two number
            WYJSCIE
            SOM negative_test
            STP

        negative_test:
            POB negative
            WYJSCIE
            STP

        number: RST 21
        negative: RST -10
    "#;

    // full pipeline test
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let machine_code = assemble_program(&ast).unwrap();

    let mut machine = asmachina::MachineW::new();
    machine.load_program(&machine_code).unwrap();
    machine.run().unwrap();

    assert_eq!(machine.get_output_buffer(), &[42]); // 21 * 2

    // disassembly test
    let disassembled = disassemble(&machine_code).unwrap();
    let disasm_text = disassembled.join("\n");
    
    assert!(disasm_text.contains("POB"));
    assert!(disasm_text.contains("DOD"));
    assert!(disasm_text.contains("WYJSCIE"));
    assert!(disasm_text.contains("SOM"));
    assert!(disasm_text.contains("STP"));
}
