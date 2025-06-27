use asmachina::MachineW;

#[test]
fn test_interactive_character_io() {
    let mut machine = MachineW::new();
    machine.set_interactive_mode(false);
    
    // WPR (character input)
    machine.set_input_buffer(vec![65, 66, 67]); // ASCII: 'A', 'B', 'C'
    
    let wpr_instruction = (0b01110 << 11) | 0; // WPR (= WEJSCIE) 
    machine.memory[0] = wpr_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 65); // should read 'A' (ASCII 65)
    
    // WYJ (character output)
    machine.set_interactive_mode(true);
    machine.ak = 72; // 'H'
    let wyj_instruction = (0b01111 << 11) | 0; // WYJ (= WYJSCIE) 
    machine.memory[1] = wyj_instruction;
    machine.l = 1;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.get_output_buffer(), &[72]); // should output 'H'
}

#[test]
fn test_character_io_buffer_mode() {
    let mut machine = MachineW::new();
    machine.set_interactive_mode(false);
    
    machine.set_input_buffer(vec![65, 66, 67]); // ASCII 'A', 'B', 'C'
    
    let wejscie_instruction = (0b01110 << 11) | 0; // WEJSCIE
    machine.memory[0] = wejscie_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 65); // should read 'A' (ASCII 65)
    
    // Test output
    machine.ak = 72; // 'H'
    let wyjscie_instruction = (0b01111 << 11) | 0; // WYJSCIE
    machine.memory[1] = wyjscie_instruction;
    machine.l = 1;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.get_output_buffer(), &[72]); // should output 'H'
}

#[test]
fn test_simple_character_echo() {
    let mut machine = MachineW::new();
    
    // echo: WEJSCIE -> WYJSCIE -> STP
    let program = vec![
        (0b01110 << 11) | 0,    // WEJSCIE (read character)
        (0b01111 << 11) | 0,    // WYJSCIE (output character)  
        (0b00111 << 11) | 0,    // STP (stop)
    ];
    
    machine.set_input_buffer(vec![72]); // 'H'
    assert!(machine.load_program(&program).is_ok());
    
    let result = machine.run();
    assert!(result.is_ok());
    
    assert_eq!(machine.ak, 72); // should have 'H' in accumulator
    assert_eq!(machine.get_output_buffer(), &[72]); // should output 'H'
    assert_eq!(machine.is_running, false); // should have stopped
}
