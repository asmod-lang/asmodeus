use asmodeus_core::{MachineW, MachineError};

#[test]
fn test_machine_initialization() {
    let machine = MachineW::new();
    
    // memory initialization
    assert_eq!(machine.memory.len(), 2048);
    for word in &machine.memory {
        assert_eq!(*word, 0);
    }
    
    // register initialization
    assert_eq!(machine.ak, 0);
    assert_eq!(machine.l, 0);
    assert_eq!(machine.ad, 0);
    assert_eq!(machine.kod, 0);
    assert_eq!(machine.ws, 2047);
    assert_eq!(machine.is_running, false);
    assert_eq!(machine.interrupts_enabled, true);
    assert_eq!(machine.interrupt_mask, 0);
}

#[test]
fn test_memory_read_write_valid_addresses() {
    let mut machine = MachineW::new();
    
    // writing and reading at various valid addresses
    let test_cases = vec![
        (0, 0x1234),
        (1024, 0x5678),
        (2047, 0x9ABC),
        (100, 0xDEF0),
    ];
    
    for (address, value) in test_cases {
        assert!(machine.write_memory(address, value).is_ok());
        assert_eq!(machine.read_memory(address).unwrap(), value);
    }
}

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

#[test]
fn test_interrupt_disable_enable() {
    let mut machine = MachineW::new();
    
    // DNS (disable interrupts)
    machine.interrupts_enabled = true;
    let dns_instruction = (0b01000 << 11) | 0;
    machine.memory[0] = dns_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.interrupts_enabled, false);
}

#[test]
fn test_interrupt_mask_operations() {
    let mut machine = MachineW::new();
    
    // MSK (set interrupt mask)
    let msk_instruction = (0b01100 << 11) | 0xFF; // MSK instruction with mask 0xFF
    machine.memory[0] = msk_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.interrupt_mask, 0xFF);
    
    // CZM (clear interrupt mask)
    let czm_instruction = (0b01011 << 11) | 0;
    machine.memory[1] = czm_instruction;
    machine.l = 1;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.interrupt_mask, 0);
}

#[test]
fn test_interrupt_handling() {
    let mut machine = MachineW::new();
    
    // simple program
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[1] = (0b00111 << 11) | 0;  // STP
    
    // interrupt handler at address 100
    machine.memory[100] = (0b00001 << 11) | 20; // DOD 20
    machine.memory[101] = (0b01101 << 11) | 0;  // PWR (return from interrupt)
    
    machine.memory[10] = 5;   // Data for DOD 10
    machine.memory[20] = 15;  // Data for DOD 20 (in interrupt)
    
    machine.l = 0;
    machine.ak = 10;
    machine.is_running = true;
    
    // before executing first instruction
    machine.trigger_interrupt(100);
    
    // should handle interrupt first
    let initial_ws = machine.ws;
    assert!(machine.step().is_ok());
    
    // interrupt was handled?
    assert_eq!(machine.l, 100); // jumped to interrupt handler
    assert_eq!(machine.interrupts_enabled, false); // interrupts disabled
    assert_eq!(machine.ws, initial_ws - 2); // stack has saved state
    
    // verify saved state on stack
    assert_eq!(machine.memory[(initial_ws - 1) as usize], 0); // saved L (was 0)
    assert_eq!(machine.memory[initial_ws as usize], 10); // saved AK (was 10)
    
    // execute interrupt handler
    assert!(machine.step().is_ok()); // DOD 20
    assert_eq!(machine.ak, 25); // 10 + 15
    assert_eq!(machine.l, 101); // moved to next instruction
    
    assert!(machine.step().is_ok()); // PWR
    assert_eq!(machine.interrupts_enabled, true); // interrupts re-enabled
    assert_eq!(machine.ws, initial_ws); // stack restored
    assert_eq!(machine.l, 0); // returned to original L (0)
    assert_eq!(machine.ak, 10); // AK restored to original value
    
    // continue with normal execution
    assert!(machine.step().is_ok()); // DOD 10 (original instruction)
    assert_eq!(machine.ak, 15); // 10 + 5
    assert_eq!(machine.l, 1); // moved to next instruction
}

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

#[test]
fn test_conditional_jump_positive() {
    let mut machine = MachineW::new();
    
    machine.ak = 100;
    let som_instruction = (0b00110 << 11) | 500;
    machine.memory[0] = som_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 1); // no jump 
}

#[test]
fn test_conditional_jump_negative() {
    let mut machine = MachineW::new();
    
    machine.ak = 0x8000; // negative value (MSB set)
    let som_instruction = (0b00110 << 11) | 500;
    machine.memory[0] = som_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 500); // L set to target address (jump taken)
}

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

#[test]
fn test_run_steps_limit() {
    let mut machine = MachineW::new();
    
    // infinite loop program
    let program = vec![
        (0b00101 << 11) | 0,   // SOB 0: Jump to self (infinite loop)
    ];
    
    assert!(machine.load_program(&program).is_ok());
    
    // run for limited steps
    let steps = machine.run_steps(10).unwrap();
    assert_eq!(steps, 10);
    assert_eq!(machine.is_running, true); // still running
    assert_eq!(machine.l, 0); // still at address 0
}

#[test]
fn test_reset_functionality() {
    let mut machine = MachineW::new();
    
    machine.ak = 1234;
    machine.l = 100;
    machine.ws = 1000;
    machine.memory[50] = 5678;
    machine.is_running = true;
    machine.interrupts_enabled = false;
    machine.interrupt_mask = 0xFF;
    machine.registers[0] = 999;
    machine.input_buffer.push(123);
    machine.output_buffer.push(456);
    
    machine.reset();
    
    // verify
    assert_eq!(machine.ak, 0);
    assert_eq!(machine.l, 0);
    assert_eq!(machine.ad, 0);
    assert_eq!(machine.kod, 0);
    assert_eq!(machine.ws, 2047);
    assert_eq!(machine.is_running, false);
    assert_eq!(machine.interrupts_enabled, true);
    assert_eq!(machine.interrupt_mask, 0);
    assert_eq!(machine.pending_interrupt, None);
    assert_eq!(machine.registers[0], 0);
    assert_eq!(machine.memory[50], 0);
    assert!(machine.input_buffer.is_empty());
    assert!(machine.output_buffer.is_empty());
}

#[test]
fn test_memory_boundaries() {
    let mut machine = MachineW::new();
    
    // boundary addresses
    assert!(machine.write_memory(0, 0x1234).is_ok());
    assert!(machine.write_memory(2047, 0x5678).is_ok());
    assert_eq!(machine.read_memory(0).unwrap(), 0x1234);
    assert_eq!(machine.read_memory(2047).unwrap(), 0x5678);
    
    // address masking (addresses > 2047 should wrap)
    assert!(machine.write_memory(0x0800, 0x9ABC).is_ok()); // 2048 wraps to 0
    assert_eq!(machine.read_memory(0).unwrap(), 0x9ABC);
}

#[test]
fn test_stp_instruction() {
    let mut machine = MachineW::new();
    
    let stp_instruction = 0b00111 << 11;
    machine.memory[0] = stp_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.is_running, false);
    assert_eq!(machine.kod, 0b00111);
}

#[test]
fn test_dod_instruction() {
    let mut machine = MachineW::new();
    
    machine.memory[100] = 50;
    let dod_instruction = (0b00001 << 11) | 100;
    machine.memory[0] = dod_instruction;
    
    machine.ak = 25;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 75); // 25 + 50
    assert_eq!(machine.l, 1); // L incremented
}

#[test]
fn test_ode_instruction() {
    let mut machine = MachineW::new();
    
    machine.memory[100] = 30;
    let ode_instruction = (0b00010 << 11) | 100;
    machine.memory[0] = ode_instruction;
    
    machine.ak = 50;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 20); // 50 - 30
    assert_eq!(machine.l, 1); // L incremented
}

#[test]
fn test_lad_instruction() {
    let mut machine = MachineW::new();
    
    let lad_instruction = (0b00011 << 11) | 100;
    machine.memory[0] = lad_instruction;
    
    machine.ak = 1234;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.memory[100], 1234); // AK stored at address 100
    assert_eq!(machine.l, 1); // L incremented
}

#[test]
fn test_pob_instruction() {
    let mut machine = MachineW::new();
    
    machine.memory[100] = 5678;
    let pob_instruction = (0b00100 << 11) | 100;
    machine.memory[0] = pob_instruction;
    
    machine.ak = 0;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 5678); // loaded from memory
    assert_eq!(machine.l, 1); // L incremented
}

#[test]
fn test_sob_instruction() {
    let mut machine = MachineW::new();
    
    let sob_instruction = (0b00101 << 11) | 500;
    machine.memory[0] = sob_instruction;
    
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 500); // L set to target address
}

#[test]
fn test_invalid_opcode() {
    let mut machine = MachineW::new();
    
    let invalid_instruction = (0b11111 << 11) | 0; // opcode 31 (invalid)
    machine.memory[0] = invalid_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    let result = machine.step();
    assert!(matches!(result, Err(MachineError::InvalidOpcode { opcode: 31 })));
}
