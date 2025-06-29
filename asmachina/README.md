# Asmachina

**Core Machine W Emulator for Asmodeus**

```
┌────────────────────────────────────────────────────────────────────────────┐
│                                                                            │
│   █████╗ ███████╗███╗   ███╗ █████╗  ██████╗██╗  ██╗██╗███╗   ██╗ █████╗   │
│  ██╔══██╗██╔════╝████╗ ████║██╔══██╗██╔════╝██║  ██║██║████╗  ██║██╔══██╗  │
│  ███████║███████╗██╔████╔██║███████║██║     ███████║██║██╔██╗ ██║███████║  │
│  ██╔══██║╚════██║██║╚██╔╝██║██╔══██║██║     ██╔══██║██║██║╚██╗██║██╔══██║  │
│  ██║  ██║███████║██║ ╚═╝ ██║██║  ██║╚██████╗██║  ██║██║██║ ╚████║██║  ██║  │
│  ╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝  │
│                                                                            │
│                Machine W Virtual Machine & Execution Engine                │
└────────────────────────────────────────────────────────────────────────────┘
```

**Asmachina** is the core virtual machine that emulates the legendary Machine W architecture. It provides a complete 16-bit execution environment with 2048 words of memory, comprehensive instruction set support, and advanced debugging capabilities.

## 🏗️ Architecture Overview

### Machine W Specification
- **Word Size**: 16-bit architecture
- **Memory**: 2048 words (4096 bytes total)
- **Address Space**: 11-bit addressing (0-2047)
- **Instruction Format**: 16-bit words with opcode and operand fields
- **Stack**: Hardware stack growing downward from address 2047

### Register Set
```
┌─────────────────┬──────────┬─────────────────────────────────┐
│ Register        │ Size     │ Purpose                         │
├─────────────────┼──────────┼─────────────────────────────────┤
│ AK (Accumulator)│ 16-bit   │ Primary arithmetic register     │
│ L (Counter)     │ 11-bit   │ Instruction counter (PC)        │
│ AD (Address)    │ 11-bit   │ Address register for operands   │
│ KOD (Opcode)    │ 5-bit    │ Current instruction opcode      │
│ WS (Stack Ptr)  │ 11-bit   │ Stack pointer (grows downward)  │
│ R0-R7           │ 16-bit   │ General purpose registers       │
└─────────────────┴──────────┴─────────────────────────────────┘
```

### Memory Layout
```
0x0000 ┌─────────────────────────────────────┐
       │             Program Code            │
       │                                     │
       ├─────────────────────────────────────┤
       │             Data Section            │
       │                                     │
       ├─────────────────────────────────────┤
       │             Free Space              │
       │                                     │
       │                 ⋮                   │
       │                                     │
       ├─────────────────────────────────────┤
       │               Stack                 │
       │          (grows downward)           │
0x07FF └─────────────────────────────────────┘
```

## 🎯 Features

### Core Execution Engine
- **Complete Instruction Set**: All Machine W opcodes implemented
- **Extended Instructions**: MNO, DZI, MOD arithmetic operations
- **Multiple Addressing Modes**: Direct, immediate, indirect, register-based
- **Hardware Stack**: Full stack operations with overflow protection
- **Interrupt System**: Hardware interrupt handling and masking

### Advanced Features
- **Debugging Support**: Step-by-step execution with breakpoints
- **Interactive I/O**: Real-time character input/output
- **Error Handling**: Comprehensive runtime error detection
- **State Inspection**: Complete machine state access
- **Memory Protection**: Address bounds checking

### I/O System
- **Batch Mode**: Pre-loaded input buffers and output collection
- **Interactive Mode**: Real-time character-based I/O
- **Buffered Output**: Capture all program output for analysis

## 📚 Instruction Set Reference

### Arithmetic Instructions

| Opcode | Mnemonic | Format | Description |
|--------|----------|--------|-------------|
| 0001 | DOD | `DOD addr` | Add memory[addr] to AK |
| 0001 | DOD | `DOD #value` | Add immediate value to AK |
| 0010 | ODE | `ODE addr` | Subtract memory[addr] from AK |
| 0010 | ODE | `ODE #value` | Subtract immediate value from AK |

### Memory Instructions

| Opcode | Mnemonic | Format | Description |
|--------|----------|--------|-------------|
| 0011 | ŁAD/LAD | `LAD addr` | Store AK to memory[addr] |
| 0100 | POB | `POB addr` | Load memory[addr] to AK |
| 0100 | POB | `POB #value` | Load immediate value to AK |

### Control Flow Instructions

| Opcode | Mnemonic | Format | Description |
|--------|----------|--------|-------------|
| 0101 | SOB | `SOB addr` | Unconditional jump to addr |
| 0110 | SOM | `SOM addr` | Jump to addr if AK < 0 |
| 10000 | SOZ | `SOZ addr` | Jump to addr if AK = 0 |
| 0111 | STP | `STP` | Stop program execution |

### Stack Instructions

| Opcode | Mnemonic | Format | Description |
|--------|----------|--------|-------------|
| 1001 | PZS | `PZS` | Pop from stack to AK |
| 1010 | SDP | `SDP` | Push AK to stack |

### I/O Instructions

| Opcode | Mnemonic | Format | Description |
|--------|----------|--------|-------------|
| 1110 | WEJSCIE | `WEJSCIE` | Read input to AK |
| 1111 | WYJSCIE | `WYJSCIE` | Output AK value |

### Extended Instructions (Require Extended Mode)

| Opcode | Mnemonic | Format | Description |
|--------|----------|--------|-------------|
| 10001 | MNO | `MNO addr` | Multiply AK by memory[addr] |
| 10001 | MNO | `MNO #value` | Multiply AK by immediate value |
| 10010 | DZI | `DZI addr` | Divide AK by memory[addr] |
| 10010 | DZI | `DZI #value` | Divide AK by immediate value |
| 10011 | MOD | `MOD addr` | AK = AK % memory[addr] |
| 10011 | MOD | `MOD #value` | AK = AK % immediate value |

### Interrupt Instructions

| Opcode | Mnemonic | Format | Description |
|--------|----------|--------|-------------|
| 1000 | DNS | `DNS` | Disable interrupts |
| 1011 | CZM | `CZM` | Clear interrupt mask |
| 1100 | MSK | `MSK` | Set interrupt mask |
| 1101 | PWR | `PWR` | Return from interrupt |

## 🧮 Addressing Modes

### Direct Addressing
```rust
// POB 100 - Load from memory address 100
let instruction = 0x2064; // 0010 000 001100100
```

### Immediate Addressing  
```rust
// POB #42 - Load immediate value 42
let instruction = 0x212A; // 0010 001 000101010
```

### Indirect Addressing
```rust
// POB [100] - Load from memory[memory[100]]
let instruction = 0x2264; // 0010 010 001100100
```

### Register Addressing
```rust
// POB R1 - Load from register 1
let instruction = 0x2301; // 0010 011 000000001
```

## 🔧 Advanced Features

- Debugging Support (breakpoints, memory dumps)
- Memory Managment (bounds checking, size validation)
- I/O operations (WEJ, WYJ instructions)

## 🛠️ Error Handling

Asmachina provides comprehensive error detection:

```rust
use asmachina::MachineError;

match machine.step() {
    Err(MachineError::AddressOutOfBounds { address }) => {
        println!("Invalid memory access at address {}", address);
    }
    Err(MachineError::DivisionByZero { address }) => {
        println!("Division by zero at instruction {}", address);
    }
    Err(MachineError::StackOverflow) => {
        println!("Stack overflow - too many pushes");
    }
    Err(MachineError::StackUnderflow) => {
        println!("Stack underflow - pop from empty stack");
    }
    Err(MachineError::InvalidOpcode { opcode }) => {
        println!("Unknown instruction opcode: {:05b}", opcode);
    }
    Ok(()) => println!("Instruction executed successfully"),
}
```

## 📖 Examples

### Simple Calculator
```rust
use asmachina::MachineW;

let mut machine = MachineW::new();

// Program: Add 25 + 17 = 42
let program = vec![
    0x2019,  // POB 25 (address 25)
    0x0800 | 26,  // DOD 26 (add value at address 26)
    0x7800,  // WYJSCIE (output result)
    0x3800,  // STP (stop)
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // padding
    25,      // address 25: first number
    17,      // address 26: second number
];

machine.load_program(&program)?;
machine.run()?;

assert_eq!(machine.get_output_buffer(), &[42]);
```

### Stack Operations
```rust
let mut machine = MachineW::new();

// Program: Use stack to reverse two numbers
let program = vec![
    0x210A,  // POB #10 (load 10)
    0x5000,  // SDP (push to stack)
    0x2114,  // POB #20 (load 20)  
    0x5000,  // SDP (push to stack)
    0x4800,  // PZS (pop 20)
    0x7800,  // WYJSCIE (output 20)
    0x4800,  // PZS (pop 10)
    0x7800,  // WYJSCIE (output 10)
    0x3800,  // STP
];

machine.load_program(&program)?;
machine.run()?;

assert_eq!(machine.get_output_buffer(), &[20, 10]);
```

### Extended Arithmetic
```rust
// Note: Extended instructions require special handling in real usage
let mut machine = MachineW::new();

// Program: 6 * 7 = 42 (using extended MNO instruction)
let program = vec![
    0x2106,  // POB #6
    // MNO #7 would be: 0x8907 (extended instruction)
    0x7800,  // WYJSCIE
    0x3800,  // STP
];

// Extended instructions are typically handled by the assembler
// This is just for demonstration of the concept
```

## 🧪 Testing

### Unit Tests
```bash
cargo test -p asmachina
```

### Integration Tests
```bash
cargo test -p asmachina --test integration
```

### Specific Test Categories
```bash
cargo test -p asmachina instruction_tests
cargo test -p asmachina stack_tests  
cargo test -p asmachina extended_set_tests
cargo test -p asmachina error_tests
```

## 🔍 Performance Characteristics

- **Execution Speed**: ~1M instructions per second (typical)
- **Memory Usage**: ~8KB base + program size
- **Startup Time**: <1ms for new machine creation
- **Step Execution**: ~1μs per instruction (with debugging)

## 🤝 Integration with Asmodeus Pipeline

Asmachina integrates seamlessly with other Asmodeus components:

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Hephasm   │───▶│  Asmachina  │───▶│   Output    │
│ (Assembler) │    │    (VM)     │    │   Results   │
└─────────────┘    └─────────────┘    └─────────────┘
     ▲                      ▲                ▲
     │                      │                │
 AST from              Machine Code      Execution
  Parseid                                 Results
```

## 📝 API Reference

### Core Types

```rust
pub struct MachineW {
    pub memory: Vec<u16>,           // 2048 words of memory
    pub ak: u16,                    // Accumulator
    pub l: u16,                     // Instruction counter  
    pub ad: u16,                    // Address register
    pub kod: u8,                    // Opcode register
    pub ws: u16,                    // Stack pointer
    pub is_running: bool,           // Execution state
    pub registers: [u16; 8],        // General purpose registers
    // ... other fields
}

pub enum MachineError {
    AddressOutOfBounds { address: u16 },
    DivisionByZero { address: u16 },
    StackOverflow,
    StackUnderflow,
    InvalidOpcode { opcode: u8 },
    InvalidRegister { register: u8 },
    BreakpointHit { address: u16 },
    IoError { message: String },
}
```

### Key Methods

```rust
impl MachineW {
    pub fn new() -> Self;
    pub fn reset(&mut self);
    pub fn load_program(&mut self, program: &[u16]) -> Result<(), MachineError>;
    pub fn run(&mut self) -> Result<(), MachineError>;
    pub fn step(&mut self) -> Result<(), MachineError>;
    pub fn step_instruction(&mut self) -> Result<(), MachineError>;
    
    // Memory operations
    pub fn read_memory(&self, address: u16) -> Result<u16, MachineError>;
    pub fn write_memory(&mut self, address: u16, value: u16) -> Result<(), MachineError>;
    
    // I/O operations  
    pub fn set_input_buffer(&mut self, inputs: Vec<u16>);
    pub fn get_output_buffer(&self) -> &[u16];
    pub fn set_interactive_mode(&mut self, enabled: bool);
    
    // Debugging
    pub fn add_breakpoint(&mut self, address: u16) -> Result<(), MachineError>;
    pub fn remove_breakpoint(&mut self, address: u16) -> bool;
    pub fn get_current_state(&self) -> MachineWState;
}
```

## 🔗 Related Components

- **[Hephasm](../hephasm/)** - Assembler that generates machine code for Asmachina
- **[Dismael](../dismael/)** - Disassembler that converts machine code back to assembly
- **[Shared](../shared/)** - Common types and utilities used by Asmachina

## 📜 License

This crate is part of the Asmodeus project and is licensed under the MIT License.

---

**Asmachina - Where Machine W Lives Again** 🚀
