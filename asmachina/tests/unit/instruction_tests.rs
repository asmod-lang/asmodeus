use asmachina::{MachineW, MachineError};

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
    
    let sob_instruction = (0b00101 << 11) | 200;
    machine.memory[0] = sob_instruction;
    
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 200); // L set to target address
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
fn test_soz_instruction_jump_when_zero() {
    let mut machine = MachineW::new();
    
    machine.ak = 0;
    let soz_instruction = (0b10000 << 11) | 200;
    machine.memory[0] = soz_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 200); // L set to target address (jump taken)
}

#[test]
fn test_soz_instruction_no_jump_when_nonzero() {
    let mut machine = MachineW::new();
    
    machine.ak = 100;
    let soz_instruction = (0b10000 << 11) | 200;
    machine.memory[0] = soz_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 1); // L incremented normally (no jump)
}

#[test]
fn test_soz_instruction_no_jump_when_negative() {
    let mut machine = MachineW::new();
    
    machine.ak = 0x8000;
    let soz_instruction = (0b10000 << 11) | 200;
    machine.memory[0] = soz_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 1); // L incremented normally (no jump)
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

#[test]
fn test_conditional_jump_positive() {
    let mut machine = MachineW::new();
    
    machine.ak = 100;
    let som_instruction = (0b00110 << 11) | 200;
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
    let som_instruction = (0b00110 << 11) | 200;
    machine.memory[0] = som_instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.l, 200); // L set to target address (jump taken)
}
