//! integration tests for asmachina module

use asmachina::{MachineW};

#[test]
fn test_complex_program_with_stack_and_io() {
    let mut machine = MachineW::new();
    
    // uses stack to calculate sum and output result
    let program = vec![
        (0b00100 << 11) | 20,  // POB 20: Load first number
        (0b01010 << 11) | 0,   // SDP: Push to stack
        (0b00100 << 11) | 21,  // POB 21: Load second number
        (0b01001 << 11) | 0,   // PZS: Pop first number
        (0b00001 << 11) | 21,  // DOD 21: Add second number
        (0b01111 << 11) | 0,   // WYJSCIE: Output result
        (0b00111 << 11) | 0,   // STP: Stop
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // Padding to address 20
        25,                    // Address 20: first number
        35,                    // Address 21: second number
    ];
    
    assert!(machine.load_program(&program).is_ok());
    assert!(machine.run().is_ok());
    
    assert_eq!(machine.ak, 60); // 25 + 35
    assert_eq!(machine.get_output_buffer(), &[60]);
    assert_eq!(machine.is_running, false);
}
