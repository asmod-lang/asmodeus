use asmachina::{MachineW, MachineError};

#[test]
fn test_arithmetic_overflow() {
    let mut machine = MachineW::new();
    
    // addition overflow
    machine.ak = 0xFFFF; // max u16 value
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[10] = 1; // add 1 to cause overflow
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 0); // wrapped to 0
    
    // subtraction underflow
    machine.ak = 0;
    machine.memory[1] = (0b00010 << 11) | 11; // ODE 11
    machine.memory[11] = 1; // subtract 1 to cause underflow
    machine.l = 1;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 0xFFFF); // wrapped to max value
}
