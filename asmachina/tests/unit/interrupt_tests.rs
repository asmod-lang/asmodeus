use asmachina::{MachineW};

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
