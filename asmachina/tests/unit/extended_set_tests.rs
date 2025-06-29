use asmachina::{MachineW, MachineError};

#[test]
fn test_mno_instruction_basic() {
    let mut machine = MachineW::new();
    
    // basic multiplication: 6 * 7 = 42
    machine.memory[100] = 7;
    let mno_instruction = (0b10001 << 11) | 100; // MNO 100
    machine.memory[0] = mno_instruction;
    
    machine.ak = 6;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 42);
    assert_eq!(machine.l, 1); // L incremented
}

#[test]
fn test_mno_instruction_immediate() {
    let mut machine = MachineW::new();
    
    // immediate multiplication: 8 * 5 = 40
    let mno_instruction = (0b10001 << 11) | (0b001 << 8) | 5; // MNO #5
    machine.memory[0] = mno_instruction;
    
    machine.ak = 8;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 40);
}

#[test]
fn test_mno_instruction_zero() {
    let mut machine = MachineW::new();
    
    // multiplication by zero
    let mno_instruction = (0b10001 << 11) | (0b001 << 8) | 0; // MNO #0
    machine.memory[0] = mno_instruction;
    
    machine.ak = 100;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 0);
}

#[test]
fn test_mno_instruction_overflow() {
    let mut machine = MachineW::new();
    
    // multiplication overflow (wrapping behavior)
    let mno_instruction = (0b10001 << 11) | (0b001 << 8) | 2; // MNO #2
    machine.memory[0] = mno_instruction;
    
    machine.ak = 40000; // will overflow when multiplied by 2
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 40000u16.wrapping_mul(2));
}

#[test]
fn test_dzi_instruction_basic() {
    let mut machine = MachineW::new();
    
    // basic division: 42 / 6 = 7
    machine.memory[100] = 6;
    let dzi_instruction = (0b10010 << 11) | 100; // DZI 100
    machine.memory[0] = dzi_instruction;
    
    machine.ak = 42;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 7);
    assert_eq!(machine.l, 1); // L incremented
}

#[test]
fn test_dzi_instruction_immediate() {
    let mut machine = MachineW::new();
    
    // immediate division: 100 / 4 = 25
    let dzi_instruction = (0b10010 << 11) | (0b001 << 8) | 4; // DZI #4
    machine.memory[0] = dzi_instruction;
    
    machine.ak = 100;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 25);
}

#[test]
fn test_dzi_instruction_division_by_zero() {
    let mut machine = MachineW::new();
    
    // division by zero error
    let dzi_instruction = (0b10010 << 11) | (0b001 << 8) | 0; // DZI #0
    machine.memory[0] = dzi_instruction;
    
    machine.ak = 42;
    machine.l = 0;
    machine.is_running = true;
    
    let result = machine.step();
    assert!(matches!(result, Err(MachineError::DivisionByZero)));
}

#[test]
fn test_dzi_instruction_integer_division() {
    let mut machine = MachineW::new();
    
    // integer division with remainder: 10 / 3 = 3 (remainder 1)
    let dzi_instruction = (0b10010 << 11) | (0b001 << 8) | 3; // DZI #3
    machine.memory[0] = dzi_instruction;
    
    machine.ak = 10;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 3); // int division
}

#[test]
fn test_mod_instruction_basic() {
    let mut machine = MachineW::new();
    
    // basic modulo: 17 % 5 = 2
    machine.memory[100] = 5;
    let mod_instruction = (0b10011 << 11) | 100; // MOD 100
    machine.memory[0] = mod_instruction;
    
    machine.ak = 17;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 2);
    assert_eq!(machine.l, 1); // L incremented
}

#[test]
fn test_mod_instruction_immediate() {
    let mut machine = MachineW::new();
    
    // immediate modulo: 23 % 7 = 2
    let mod_instruction = (0b10011 << 11) | (0b001 << 8) | 7; // MOD #7
    machine.memory[0] = mod_instruction;
    
    machine.ak = 23;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 2);
}

#[test]
fn test_mod_instruction_modulo_by_zero() {
    let mut machine = MachineW::new();
    
    // modulo by zero error
    let mod_instruction = (0b10011 << 11) | (0b001 << 8) | 0; // MOD #0
    machine.memory[0] = mod_instruction;
    
    machine.ak = 42;
    machine.l = 0;
    machine.is_running = true;
    
    let result = machine.step();
    assert!(matches!(result, Err(MachineError::DivisionByZero)));
}

#[test]
fn test_mod_instruction_zero_dividend() {
    let mut machine = MachineW::new();
    
    // 0 % n = 0
    let mod_instruction = (0b10011 << 11) | (0b001 << 8) | 5; // MOD #5
    machine.memory[0] = mod_instruction;
    
    machine.ak = 0;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 0);
}

#[test]
fn test_mod_instruction_same_numbers() {
    let mut machine = MachineW::new();
    
    // n % n = 0
    let mod_instruction = (0b10011 << 11) | (0b001 << 8) | 42; // MOD #42
    machine.memory[0] = mod_instruction;
    
    machine.ak = 42;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 0);
}

#[test]
fn test_extended_instructions_with_different_addressing_modes() {
    let mut machine = MachineW::new();
    
    // memory for indirect addressing
    machine.memory[50] = 200; // pointer
    machine.memory[200] = 6;  // actual value
    
    // MNO with indirect addressing: 7 * 6 = 42
    let mno_indirect = (0b10001 << 11) | (0b010 << 8) | 50; // MNO [50]
    machine.memory[0] = mno_indirect;
    
    machine.ak = 7;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.step().is_ok());
    assert_eq!(machine.ak, 42);
}

#[test]
fn test_extended_arithmetic_sequence() {
    let mut machine = MachineW::new();
    
    // start with 10, multiply by 3, divide by 2, modulo 7
    // 10 * 3 = 30
    let mno_instruction = (0b10001 << 11) | (0b001 << 8) | 3; // MNO #3
    machine.memory[0] = mno_instruction;
    
    // 30 / 2 = 15
    let dzi_instruction = (0b10010 << 11) | (0b001 << 8) | 2; // DZI #2
    machine.memory[1] = dzi_instruction;
    
    // 15 % 7 = 1
    let mod_instruction = (0b10011 << 11) | (0b001 << 8) | 7; // MOD #7
    machine.memory[2] = mod_instruction;
    
    let stp_instruction = (0b00111 << 11) | 0; // STP
    machine.memory[3] = stp_instruction;
    
    machine.ak = 10;
    machine.l = 0;
    machine.is_running = true;
    
    assert!(machine.run().is_ok());
    assert_eq!(machine.ak, 1); // 10 * 3 / 2 % 7 = 1
    assert_eq!(machine.is_running, false);
}
