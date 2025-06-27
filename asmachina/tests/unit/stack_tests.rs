use asmachina::{MachineW, MachineError};

#[test]
fn test_stack_operations_basic() {
    let mut machine = MachineW::new();
    
    // SDP (push)
    machine.ak = 1234;
    let sdp_instruction = (0b01010 << 11) | 0; // SDP
    machine.memory[0] = sdp_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    let initial_ws = machine.ws;
    assert!(machine.step().is_ok());
    assert_eq!(machine.memory[initial_ws as usize], 1234); // pushed to stack
    assert_eq!(machine.ws, initial_ws - 1); // WS decremented
    
    // PZS (pop)
    let pzs_instruction = (0b01001 << 11) | 0; // PZS
    machine.memory[1] = pzs_instruction;
    machine.l = 1;
    machine.ak = 0; // clear AK
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 1234); // popped from stack
    assert_eq!(machine.ws, initial_ws); // WS incremented back
}

#[test]
fn test_stack_operations_multiple_values() {
    let mut machine = MachineW::new();
    
    // push multiple values
    let values = vec![100, 200, 300, 400];
    machine.is_running = true;
    
    for (i, &value) in values.iter().enumerate() {
        machine.ak = value;
        machine.memory[i] = (0b01010 << 11) | 0; // SDP
        machine.l = i as u16;
        assert!(machine.step().is_ok());
    }
    
    // pop values in reverse order
    for (i, &expected_value) in values.iter().rev().enumerate() {
        let addr = values.len() + i;
        machine.memory[addr] = (0b01001 << 11) | 0; // PZS
        machine.l = addr as u16;
        machine.ak = 0; // clear AK
        assert!(machine.step().is_ok());
        assert_eq!(machine.ak, expected_value);
    }
}

#[test]
fn test_stack_overflow() {
    let mut machine = MachineW::new();
    
    machine.ws = 0; // stack pointer to bottom
    machine.ak = 1234;
    
    let sdp_instruction = (0b01010 << 11) | 0;
    machine.memory[0] = sdp_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    let result = machine.step();
    assert!(matches!(result, Err(MachineError::StackOverflow)));
}

#[test]
fn test_stack_underflow() {
    let mut machine = MachineW::new();
    
    machine.ws = 2047; // stack is empty
    
    let pzs_instruction = (0b01001 << 11) | 0;
    machine.memory[0] = pzs_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    let result = machine.step();
    assert!(matches!(result, Err(MachineError::StackUnderflow)));
}
