use dismael::disassemble;

#[test]
fn test_large_program() {
    // scalability
    let mut machine_code = Vec::new();
    
    for i in 0..100 {
        if i % 10 == 0 {
            // jumps every 10 instructions
            machine_code.push((0b00101 << 11) | ((i + 5) % 100));
        } else if i % 10 == 5 {
            // data
            machine_code.push(i * 2);
        } else {
            // instructions
            machine_code.push((0b00001 << 11) | ((i + 1) % 100));
        }
    }

    let result = disassemble(&machine_code);
    assert!(result.is_ok());
    
    let disasm = result.unwrap();
    assert!(disasm.len() > 100); // at least as many lines as instructions
}
