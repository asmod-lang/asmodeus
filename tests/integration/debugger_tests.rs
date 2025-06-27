use asmachina::{MachineW, MachineError};

#[test]
fn test_debugger_breakpoint_functionality() {
    let mut machine = MachineW::new();
    
    let program = vec![
        (0b00100 << 11) | 10,  // POB 10
        (0b00001 << 11) | 11,  // DOD 11  
        (0b00111 << 11) | 0,   // STP
        0, 0, 0, 0, 0, 0, 0,   // padding
        25,                    // address 10: value 25
        17,                    // address 11: value 17
    ];
    
    assert!(machine.load_program(&program).is_ok());
    machine.is_running = true;
    
    // breakpoint at address 1 (DOD instruction)
    assert!(machine.add_breakpoint(1).is_ok());
    assert!(machine.has_breakpoint(1));
    
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 1 })));
    assert_eq!(machine.l, 1); // stopped at breakpoint
    assert_eq!(machine.ak, 25); // POB executed, AK = 25
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 2); // moved to STP
    assert_eq!(machine.ak, 42); // 25 + 17 = 42
    
    // should stop at STP
    assert!(machine.run_until_halt_or_breakpoint().is_ok());
    assert_eq!(machine.is_running, false); // stopped
}

#[test]
fn test_debugger_state_inspection() {
    let mut machine = MachineW::new();
    
    machine.ak = 0x1234;
    machine.l = 0x10;
    machine.ws = 2000;
    machine.memory[0x10] = 0x5678;
    machine.memory[0x11] = 0x9ABC;
    
    let state = machine.get_current_state();
    assert_eq!(state.ak, 0x1234);
    assert_eq!(state.l, 0x10);
    assert_eq!(state.ws, 2000);
    
    // Test memory range - returns Option<Vec<(address, value)>>
    let memory_range = machine.get_memory_range(0x10, 0x11);
    assert!(memory_range.is_some());
    let range = memory_range.unwrap();
    assert_eq!(range.len(), 2);
    assert_eq!(range[0], (0x10, 0x5678)); // (address, value) pairs
    assert_eq!(range[1], (0x11, 0x9ABC));
}

#[test]
fn test_debugger_breakpoint_management() {
    let mut machine = MachineW::new();
    
    // multiple breakpoints
    assert!(machine.add_breakpoint(10).is_ok());
    assert!(machine.add_breakpoint(20).is_ok());
    assert!(machine.add_breakpoint(30).is_ok());
    
    assert!(machine.has_breakpoint(10));
    assert!(machine.has_breakpoint(20));
    assert!(machine.has_breakpoint(30));
    assert!(!machine.has_breakpoint(40));
    
    let breakpoints = machine.list_breakpoints();
    assert_eq!(breakpoints.len(), 3);
    assert!(breakpoints.contains(&10));
    assert!(breakpoints.contains(&20));
    assert!(breakpoints.contains(&30));
    
    machine.remove_breakpoint(20);
    assert!(!machine.has_breakpoint(20));
    assert_eq!(machine.list_breakpoints().len(), 2);
    
    // clear all
    machine.clear_all_breakpoints();
    assert_eq!(machine.list_breakpoints().len(), 0);
}

#[test]
fn test_debugger_step_instruction() {
    let mut machine = MachineW::new();
    
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[1] = (0b00011 << 11) | 11; // LAD 11 
    machine.memory[2] = (0b00111 << 11) | 0;  // STP
    machine.memory[10] = 15; // data
    
    machine.l = 0;
    machine.ak = 5;
    machine.is_running = true;
    
    // DOD 10
    assert!(machine.step_instruction().is_ok());
    assert_eq!(machine.l, 1);
    assert_eq!(machine.ak, 20); // 5 + 15
    
    // LAD 11
    assert!(machine.step_instruction().is_ok());
    assert_eq!(machine.l, 2);
    assert_eq!(machine.memory[11], 20); // AK stored
    
    // STP
    assert!(machine.step_instruction().is_ok());
    assert_eq!(machine.l, 3);
    assert_eq!(machine.is_running, false);
}

#[test]
fn test_debugger_run_until_breakpoint_with_multiple_breakpoints() {
    let mut machine = MachineW::new();
    
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[1] = (0b00101 << 11) | 3;  // SOB 3 (jump to address 3)
    machine.memory[2] = (0b00111 << 11) | 0;  // STP (shouldn't reach)
    machine.memory[3] = (0b00001 << 11) | 11; // DOD 11
    machine.memory[4] = (0b00111 << 11) | 0;  // STP
    machine.memory[10] = 5;
    machine.memory[11] = 10;
    
    machine.l = 0;
    machine.ak = 0;
    machine.is_running = true;
    
    assert!(machine.add_breakpoint(1).is_ok()); // at SOB
    assert!(machine.add_breakpoint(3).is_ok()); // at DOD 11
    
    // first breakpoint (should hit address 1)
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 1 })));
    assert_eq!(machine.l, 1);
    assert_eq!(machine.ak, 5); // DOD 10 executed
    
    // SOB instruction manually at address 1
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 3); // jumped to address 3
    assert_eq!(machine.ak, 5); // still 5
    
    // second breakpoint (should hit address 3)
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 3 })));
    assert_eq!(machine.l, 3);
    assert_eq!(machine.ak, 5);
    
    // DOD instruction manually at address 3
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 4); // moved to STP
    assert_eq!(machine.ak, 15); // 5 + 10
    
    let result = machine.run_until_halt_or_breakpoint();
    assert!(result.is_ok());
    assert_eq!(machine.is_running, false);
    assert_eq!(machine.ak, 15); // 5 + 10
}

#[test]
fn test_breakpoint_hit_at_exact_address() {
    let mut machine = MachineW::new();
    
    machine.memory[0] = (0b00001 << 11) | 10; // DOD 10
    machine.memory[1] = (0b00001 << 11) | 11; // DOD 11
    machine.memory[2] = (0b00111 << 11) | 0;  // STP
    machine.memory[10] = 5;
    machine.memory[11] = 7;
    
    assert!(machine.add_breakpoint(1).is_ok());
    machine.l = 0;
    machine.ak = 0;
    machine.is_running = true;
    
    // should stop exactly at breakpoint
    let result = machine.run_until_halt_or_breakpoint();
    assert!(matches!(result, Err(MachineError::BreakpointHit { address: 1 })));
    assert_eq!(machine.l, 1); // stopped AT the breakpoint
}

#[test]
fn test_memory_range_retrieval() {
    let mut machine = MachineW::new();
    
    for i in 0..20 {
        machine.memory[i] = (i * 2) as u16;
    }
    
    let range = machine.get_memory_range(5, 10);
    assert!(range.is_some());
    let range = range.unwrap();
    assert_eq!(range.len(), 6); // inclusive range: 5,6,7,8,9,10
    assert_eq!(range[0], (5, 10));   // (address, value): memory[5] = 5*2 = 10
    assert_eq!(range[1], (6, 12));   // memory[6] = 6*2 = 12
    assert_eq!(range[5], (10, 20));  // memory[10] = 10*2 = 20
}

#[test]
fn test_debugger_state_snapshot() {
    let mut machine = MachineW::new();
    
    machine.ak = 0xABCD;
    machine.l = 0x123;
    machine.ad = 0x456;
    machine.kod = 0x7;
    machine.ws = 1500;
    
    let state = machine.get_current_state();
    
    assert_eq!(state.ak, 0xABCD);
    assert_eq!(state.l, 0x123);
    assert_eq!(state.ad, 0x456);
    assert_eq!(state.kod, 0x7);
    assert_eq!(state.ws, 1500);
    assert_eq!(state.is_running, false); // default state
}
