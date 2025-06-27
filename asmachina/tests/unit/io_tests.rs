use asmachina::{MachineW};

#[test]
fn test_io_operations_with_buffer() {
    let mut machine = MachineW::new();
    
    // input with buffer
    machine.set_input_buffer(vec![42, 100]);
    
    let wejscie_instruction = (0b01110 << 11) | 0; // WEJSCIE instruction
    machine.memory[0] = wejscie_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 42); // first input value
    
    // output
    machine.ak = 123;
    let wyjscie_instruction = (0b01111 << 11) | 0; // WYJSCIE instruction
    machine.memory[1] = wyjscie_instruction;
    machine.l = 1;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.get_output_buffer(), &[123]);
}
