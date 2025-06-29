use asmachina::{MachineW, MachineError};

#[test]
fn test_direct_addressing() {
    let mut machine = MachineW::new();
    
    // memory[100] = 42
    machine.memory[100] = 42;
    
    // DOD 100 with direct addressing (mode 000)
    let instruction = (0b00001 << 11) | (0b000 << 8) | 100;
    machine.memory[0] = instruction;
    
    machine.ak = 10;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 52); // 10 + 42
}

#[test]
fn test_immediate_addressing() {
    let mut machine = MachineW::new();
    
    // DOD #42 with immediate addressing (mode 001)
    let instruction = (0b00001 << 11) | (0b001 << 8) | 42;
    machine.memory[0] = instruction;
    
    machine.ak = 10;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 52); // 10 + 42 (immediate)
}

#[test]
fn test_indirect_addressing() {
    let mut machine = MachineW::new();
    
    // memory[50] = 200, memory[200] = 33
    machine.memory[50] = 200;
    machine.memory[200] = 33;
    
    // DOD [50] with indirect addressing (mode 010)
    let instruction = (0b00001 << 11) | (0b010 << 8) | 50;
    machine.memory[0] = instruction;
    
    machine.ak = 7;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 40); // 7 + 33
}

#[test]
fn test_register_addressing() {
    let mut machine = MachineW::new();
    
    // R2 contains address 150, memory[150] = 25
    machine.registers[2] = 150;
    machine.memory[150] = 25;
    
    // DOD R2 with register addressing (mode 100), register 2
    let instruction = (0b00001 << 11) | (0b100 << 8) | 2;
    machine.memory[0] = instruction;
    
    machine.ak = 5;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 30); // 5 + 25
}

#[test]
fn test_register_indirect_addressing() {
    let mut machine = MachineW::new();
    
    // R1 = 300, memory[300] = 77
    machine.registers[1] = 300;
    machine.memory[300] = 77;
    
    // POB [R1] with register indirect addressing (mode 101), register 1
    let instruction = (0b00100 << 11) | (0b101 << 8) | 1;
    machine.memory[0] = instruction;
    
    machine.ak = 0;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 77);
}

#[test]
fn test_base_register_addressing() {
    let mut machine = MachineW::new();
    
    // R3 = 1000, memory[1005] = 88 (base + offset)
    machine.registers[3] = 1000;
    machine.memory[1005] = 88;
    
    // POB R3[5] with base register addressing (mode 110)
    // encode register 3 in bits 6-8 and offset 5 in bits 0-5
    let encoded_arg = (3 << 6) | 5;
    let instruction = (0b00100 << 11) | (0b110 << 8) | encoded_arg;
    machine.memory[0] = instruction;
    
    machine.ak = 0;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 88);
}

#[test]
fn test_relative_addressing_positive() {
    let mut machine = MachineW::new();
    
    // memory[5] = 99 (current L=0, +5 = address 5)
    machine.memory[5] = 99;
    
    // DOD +5 with relative addressing (mode 111)
    let instruction = (0b00001 << 11) | (0b111 << 8) | 5;
    machine.memory[0] = instruction;
    
    machine.ak = 1;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 100); // 1 + 99
}

#[test]
fn test_relative_addressing_negative() {
    let mut machine = MachineW::new();
    
    // Place instruction at address 10, memory[8] = 15
    machine.memory[8] = 15;
    
    // DOD -2 with relative addressing (mode 111)
    // negative offset: encode -2 as 0x7FE (sign extend)
    let negative_offset = ((-2i16 as u16) & 0xFF) | 0x100; // sign bit in argument
    let instruction = (0b00001 << 11) | (0b111 << 8) | (negative_offset & 0xFF);
    machine.memory[10] = instruction;
    
    machine.ak = 5;
    machine.l = 10;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 20); // 5 + 15 (from address 10-2=8)
}

#[test]
fn test_multiple_indirect_addressing() {
    let mut machine = MachineW::new();
    
    // memory[60] = 70, memory[70] = 80, memory[80] = 55
    machine.memory[60] = 70;
    machine.memory[70] = 80;
    machine.memory[80] = 55;
    
    // DOD [[60]] with multiple indirect addressing (mode 011)
    let instruction = (0b00001 << 11) | (0b011 << 8) | 60;
    machine.memory[0] = instruction;
    
    machine.ak = 45;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 100); // 45 + 55
}

#[test]
fn test_addressing_mode_bounds_checking() {
    let mut machine = MachineW::new();
    
    // invalid register number
    let instruction = (0b00001 << 11) | (0b100 << 8) | 8; // Register 8 (max is 7)
    machine.memory[0] = instruction;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_err());
}
