# Hephasm

**Assembler for Asmodeus Language**

```
┌───────────────────────────────────────────────────────────────┐
│                                                               │
│  ██╗  ██╗███████╗██████╗ ██╗  ██╗ █████╗ ███████╗███╗   ███╗  │
│  ██║  ██║██╔════╝██╔══██╗██║  ██║██╔══██╗██╔════╝████╗ ████║  │
│  ███████║█████╗  ██████╔╝███████║███████║███████╗██╔████╔██║  │
│  ██╔══██║██╔══╝  ██╔═══╝ ██╔══██║██╔══██║╚════██║██║╚██╔╝██║  │
│  ██║  ██║███████╗██║     ██║  ██║██║  ██║███████║██║ ╚═╝ ██║  │
│  ╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝  │
│                                                               │
│             AST Converter for Asmodeus Language               │
└───────────────────────────────────────────────────────────────┘
```

**Hephasm** is the assembler component of the Asmodeus toolchain. It takes the Abstract Syntax Tree (AST) from Parseid and generates binary machine code that can be executed on the Machine W virtual machine (Asmachina). Features multi-pass assembly, macro expansion, symbol resolution, and extended instruction set support.

## 🎯 Features

### Core Assembly Capabilities
- **Multi-Pass Assembly**: Three-pass assembler for complete symbol resolution
- **Macro Expansion**: Full macro system with parameter substitution
- **Symbol Table Management**: Forward and backward label references
- **Extended Instruction Set**: Support for MNO, DZI, MOD operations
- **Multiple Addressing Modes**: All Machine W addressing modes supported
- **Directive Processing**: Data definition and memory reservation

### Advanced Features
- **Error Reporting**: Detailed error messages with line numbers
- **Optimization**: Basic code optimization during assembly
- **Binary Generation**: Compact 16-bit machine code output
- **Address Validation**: Bounds checking for all memory references
- **Type Safety**: Operand type validation and conversion

## 🚀 Quick Start

### Basic Usage

```rust
use hephasm::{assemble_source, assemble_program};
use parseid::parse_source;

// Assemble from source code directly
let source = r#"
    start:
        POB #42     ; Load immediate value
        WYJSCIE     ; Output the value
        STP         ; Stop program
"#;

let machine_code = assemble_source(source)?;
println!("Generated {} words of machine code", machine_code.len());

// Or assemble from AST
let ast = parse_source(source)?;
let machine_code = assemble_program(&ast)?;
```

### Extended Instruction Set

```rust
use hephasm::assemble_source_extended;

let extended_program = r#"
    ; Extended arithmetic operations
    start:
        POB #15     ; Load 15
        MNO #3      ; Multiply by 3 (45)
        DZI #5      ; Divide by 5 (9)
        MOD #7      ; Modulo 7 (2)
        WYJSCIE     ; Output result
        STP
"#;

// Enable extended instruction set
let machine_code = assemble_source_extended(extended_program, true)?;
```

### Examining Generated Code

```rust
let source = r#"
    main:
        POB data
        DOD #10
        WYJSCIE
        STP
    data: RST 42
"#;

let machine_code = assemble_source(source)?;

// Print generated instructions in hex
for (addr, word) in machine_code.iter().enumerate() {
    println!("0x{:04X}: 0x{:04X} ({})", addr, word, word);
}

// Expected output:
// 0x0000: 0x2004 (8196)  -- POB 4 (direct addressing)
// 0x0001: 0x090A (2314)  -- DOD #10 (immediate addressing)  
// 0x0002: 0x7800 (30720) -- WYJSCIE
// 0x0003: 0x3800 (14336) -- STP
// 0x0004: 0x002A (42)    -- data: RST 42
```

## 🏗️ Assembly Process

### Three-Pass Assembly

Hephasm uses a sophisticated three-pass assembly process:

```
Pass 1: Macro Expansion
├── Collect macro definitions
├── Expand macro calls with parameter substitution
└── Generate expanded program without macros

Pass 2: Symbol Table Building  
├── Scan all labels and data definitions
├── Calculate addresses for all symbols
├── Build complete symbol table
└── Validate symbol references

Pass 3: Code Generation
├── Process instructions into machine code
├── Resolve all symbol references
├── Apply addressing mode encoding
└── Generate final binary output
```

### Instruction Encoding

Machine W instructions use 16-bit encoding:

```
┌─────────────┬─────────────┬─────────────────────────┐
│   Opcode    │ Addr Mode   │       Operand           │
│   (5 bits)  │  (3 bits)   │      (8 bits)           │
└─────────────┴─────────────┴─────────────────────────┘
 15         11 10          8 7                       0
```

## 🔧 API Reference

### Main Functions

```rust
// Assemble from source code
pub fn assemble_source(source: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>>;

// Assemble with extended instruction set
pub fn assemble_source_extended(source: &str, extended_mode: bool) 
    -> Result<Vec<u16>, Box<dyn std::error::Error>>;

// Assemble from AST
pub fn assemble_program(program: &Program) -> Result<Vec<u16>, AssemblerError>;
pub fn assemble_program_extended(program: &Program, extended_mode: bool) 
    -> Result<Vec<u16>, AssemblerError>;
```

### Assembler Class

For advanced usage and control:

```rust
use hephasm::Assembler;

let mut assembler = Assembler::new();
// or with extended instruction set
let mut assembler = Assembler::new_with_extended(true);

let machine_code = assembler.assemble(&ast)?;
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum AssemblerError {
    #[error("Undefined symbol '{symbol}' at line {line}")]
    UndefinedSymbol { symbol: String, line: usize },
    
    #[error("Invalid opcode '{opcode}' at line {line}")]
    InvalidOpcode { opcode: String, line: usize },
    
    #[error("Address out of bounds: {address} at line {line}")]
    AddressOutOfBounds { address: u16, line: usize },
    
    #[error("Extended instruction '{instruction}' not enabled at line {line}")]
    ExtendedInstructionNotEnabled { instruction: String, line: usize },
    
    #[error("Invalid addressing mode for instruction at line {line}")]
    InvalidAddressingMode { line: usize },
    
    #[error("Macro '{name}' already defined at line {line}")]
    DuplicateMacro { name: String, line: usize },
    
    #[error("Parser error: {0}")]
    ParserError(#[from] parseid::ParserError),
}
```

## 📖 Examples

### Basic Assembly

```rust
use hephasm::assemble_source;

let basic_program = r#"
    ; Simple addition program
    start:
        POB first       ; Load first number
        DOD second      ; Add second number
        WYJSCIE         ; Output result
        STP             ; Stop

    first:  RST 25      ; Data: 25
    second: RST 17      ; Data: 17
"#;

let machine_code = assemble_source(basic_program)?;

// Verify the generated code
assert_eq!(machine_code.len(), 6);

// Check instruction encoding
// POB first (address 4) -> direct addressing
let pob_instruction = machine_code[0];
let opcode = (pob_instruction >> 11) & 0b11111;
let addr_mode = (pob_instruction >> 8) & 0b111;
let operand = pob_instruction & 0xFF;

assert_eq!(opcode, 0b00100);  // POB opcode
assert_eq!(addr_mode, 0b000); // Direct addressing
assert_eq!(operand, 4);       // Address of 'first'
```

### Macro Assembly

```rust
let macro_program = r#"
    ; Define a macro for adding two values
    MAKRO add_values val1 val2
        POB val1
        DOD val2
        WYJSCIE
    KONM
    
    ; Define another macro with complex logic
    MAKRO conditional_add condition value
        POB condition
        SOM skip_add
        POB result
        DOD value
        LAD result
    skip_add:
        ; Continue...
    KONM
    
    start:
        add_values #10 #20      ; Expands to POB #10, DOD #20, WYJSCIE
        conditional_add flag data_value
        STP
        
    flag: RST 1
    data_value: RST 15
    result: RPA
"#;

let machine_code = assemble_source(macro_program)?;

// The assembler will expand macros and resolve all symbols
println!("Macro program assembled to {} words", machine_code.len());
```

### Extended Instruction Assembly

```rust
use hephasm::assemble_source_extended;

let extended_program = r#"
    ; Factorial calculation using extended instructions
    start:
        POB n           ; Load 5
        LAD counter     ; counter = 5
        POB one         ; result = 1
        LAD result
        
    factorial_loop:
        POB counter     ; if counter == 0, done
        SOZ done
        
        POB result      ; result *= counter
        MNO counter     ; Extended multiplication
        LAD result
        
        POB counter     ; counter--
        ODE one
        LAD counter
        
        SOB factorial_loop
        
    done:
        POB result      ; Output result (120)
        WYJSCIE
        STP
        
    n:       RST 5
    one:     RST 1
    counter: RPA
    result:  RPA
"#;

let machine_code = assemble_source_extended(extended_program, true)?;
```

### Addressing Mode Examples

```rust
let addressing_program = r#"
    test_addressing:
        ; Direct addressing
        POB value           ; Load from memory address
        
        ; Immediate addressing
        POB #42             ; Load literal value
        DOD #10             ; Add literal value
        
        ; Indirect addressing
        POB [pointer]       ; Load from address stored at pointer
        
        ; Register addressing (if supported)
        POB R1              ; Load from register
        LAD R2              ; Store to register
        
        STP
        
    value:   RST 100
    pointer: RST value      ; Points to 'value'
"#;

let machine_code = assemble_source(addressing_program)?;

// Each addressing mode gets encoded differently
for (i, instruction) in machine_code.iter().enumerate() {
    let addr_mode = (instruction >> 8) & 0b111;
    match addr_mode {
        0b000 => println!("Instruction {} uses direct addressing", i),
        0b001 => println!("Instruction {} uses immediate addressing", i),
        0b010 => println!("Instruction {} uses indirect addressing", i),
        0b011 => println!("Instruction {} uses register addressing", i),
        _ => {}
    }
}
```

### Data Definition and Directives

```rust
let data_program = r#"
    ; Data section with various formats
    program_start:
        POB number
        DOD hex_value
        WYJSCIE
        STP
        
    ; Data definitions
    number:     RST 42          ; Decimal
    hex_value:  RST 0x2A        ; Hexadecimal (same as 42)
    binary_val: RST 0b101010    ; Binary (same as 42)
    negative:   RST -10         ; Negative number
    
    ; Memory reservations
    buffer:     RPA             ; Reserve one word (initialized to 0)
    array:      RPA, RPA, RPA   ; Reserve three words
"#;

let machine_code = assemble_source(data_program)?;

// Data values are placed in memory after code
let code_size = 4; // 4 instructions
println!("Data starts at word {}", code_size);
println!("number = {}", machine_code[code_size]);     // 42
println!("hex_value = {}", machine_code[code_size + 1]); // 42
```

### Error Handling

```rust
use hephasm::{assemble_source, AssemblerError};

// Program with undefined symbol
let bad_program = r#"
    start:
        POB undefined_symbol    ; Error: symbol not defined
        STP
"#;

match assemble_source(bad_program) {
    Ok(_) => println!("Assembly successful"),
    Err(e) => {
        if let Some(assembler_err) = e.downcast_ref::<AssemblerError>() {
            match assembler_err {
                AssemblerError::UndefinedSymbol { symbol, line } => {
                    println!("Undefined symbol '{}' at line {}", symbol, line);
                }
                AssemblerError::AddressOutOfBounds { address, line } => {
                    println!("Address {} out of bounds at line {}", address, line);
                }
                _ => println!("Other assembler error: {}", assembler_err),
            }
        }
    }
}

// Program with extended instruction but no extended mode
let extended_without_flag = r#"
    start:
        MNO #5      ; Error: extended instruction not enabled
        STP
"#;

match assemble_source(extended_without_flag) {
    Err(e) => {
        if let Some(AssemblerError::ExtendedInstructionNotEnabled { instruction, line }) 
            = e.downcast_ref::<AssemblerError>() {
            println!("Extended instruction '{}' not enabled at line {}", 
                     instruction, line);
        }
    }
    Ok(_) => unreachable!(),
}
```

## 🧪 Testing

### Unit Tests

```bash
cargo test -p hephasm
```

### Specific Test Categories

```bash
# Test instruction assembly
cargo test -p hephasm instruction_tests

# Test addressing mode encoding
cargo test -p hephasm addressing_tests

# Test macro expansion
cargo test -p hephasm macro_tests

# Test symbol resolution
cargo test -p hephasm symbol_tests

# Test directive processing
cargo test -p hephasm directive_tests

# Test error conditions
cargo test -p hephasm error_tests
```

### Integration Tests

```bash
cargo test -p hephasm --test integration_tests
```

## 🔍 Performance Characteristics

- **Speed**: ~100K instructions per second assembly
- **Memory**: O(n) where n is program size
- **Passes**: Fixed 3-pass overhead regardless of program size
- **Symbol Resolution**: O(log n) lookup time with hash tables

### Performance Testing

```rust
use hephasm::assemble_source;
use std::time::Instant;

let large_program = include_str!("large_program.asmod");
let start = Instant::now();
let machine_code = assemble_source(large_program)?;
let duration = start.elapsed();

println!("Assembled {} lines into {} words in {:?}", 
         large_program.lines().count(), machine_code.len(), duration);
```

## 🛠️ Advanced Features

### Custom Assembler Configuration

```rust
use hephasm::Assembler;

let mut assembler = Assembler::new_with_extended(true);

// The assembler handles all configuration internally
// Extended mode enables MNO, DZI, MOD instructions
let machine_code = assembler.assemble(&ast)?;
```

### Manual Assembly Control

```rust
use hephasm::Assembler;
use parseid::parse_source;

let source = r#"
    start:
        POB data
        WYJSCIE
        STP
    data: RST 42
"#;

let ast = parse_source(source)?;
let mut assembler = Assembler::new();

// The assembler runs three passes automatically:
// 1. Macro expansion
// 2. Symbol table building  
// 3. Code generation
let machine_code = assembler.assemble(&ast)?;

println!("Final code size: {} words", machine_code.len());
```

## 🔗 Integration with Asmodeus Pipeline

Hephasm is the final transformation step before execution:

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Parseid   │───▶│   Hephasm   │───▶│  Asmachina  │
│  (Parser)   │    │ (Assembler) │    │    (VM)     │
│             │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘
        │                   │                   │
        ▼                   ▼                   ▼
   ┌─────────┐         ┌─────────┐         ┌─────────┐
   │   AST   │         │ Machine │         │Execution│
   │         │         │  Code   │         │ Results │
   └─────────┘         └─────────┘         └─────────┘
```

### Complete Pipeline Usage

```rust
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;
use asmachina::MachineW;

let source = "POB #42\nWYJSCIE\nSTP";

// Complete compilation pipeline
let tokens = tokenize(source)?;              // Lexariel
let ast = parse(tokens)?;                    // Parseid
let machine_code = assemble_program(&ast)?;  // Hephasm

// Execute the result
let mut machine = MachineW::new();
machine.load_program(&machine_code)?;       // Asmachina
machine.run()?;

assert_eq!(machine.get_output_buffer(), &[42]);
```

## 📊 Instruction Set Mapping

### Basic Instructions

| Assembly | Opcode | Encoding | Description |
|----------|--------|----------|-------------|
| `DOD addr` | 0001 | `0001_000_aaaaaaaa` | Add memory[addr] to AK |
| `DOD #val` | 0001 | `0001_001_vvvvvvvv` | Add immediate value to AK |
| `ODE addr` | 0010 | `0010_000_aaaaaaaa` | Subtract memory[addr] from AK |
| `LAD addr` | 0011 | `0011_000_aaaaaaaa` | Store AK to memory[addr] |
| `POB addr` | 0100 | `0100_000_aaaaaaaa` | Load memory[addr] to AK |
| `POB #val` | 0100 | `0100_001_vvvvvvvv` | Load immediate value to AK |
| `SOB addr` | 0101 | `0101_000_aaaaaaaa` | Jump to addr |
| `SOM addr` | 0110 | `0110_000_aaaaaaaa` | Jump to addr if AK < 0 |
| `SOZ addr` | 10000 | `10000_000_aaaaaaa` | Jump to addr if AK = 0 |
| `STP` | 0111 | `0111_000_00000000` | Stop execution |

### Extended Instructions

| Assembly | Opcode | Encoding | Description |
|----------|--------|----------|-------------|
| `MNO addr` | 10001 | `10001_000_aaaaaaa` | Multiply AK by memory[addr] |
| `MNO #val` | 10001 | `10001_001_vvvvvvv` | Multiply AK by immediate |
| `DZI addr` | 10010 | `10010_000_aaaaaaa` | Divide AK by memory[addr] |
| `DZI #val` | 10010 | `10010_001_vvvvvvv` | Divide AK by immediate |
| `MOD addr` | 10011 | `10011_000_aaaaaaa` | AK = AK % memory[addr] |
| `MOD #val` | 10011 | `10011_001_vvvvvvv` | AK = AK % immediate |

## 📜 License

This crate is part of the Asmodeus project and is licensed under the MIT License.

## 🔗 Related Components

- **[Parseid](../parseid/)** - Parser that generates AST for Hephasm
- **[Asmachina](../asmachina/)** - Virtual machine that executes Hephasm output
- **[Shared](../shared/)** - Common types and instruction encoding utilities
- **[Main Asmodeus](../)** - Complete language toolchain
