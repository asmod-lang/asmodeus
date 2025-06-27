use asmachina::{MachineW, MachineError};

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
