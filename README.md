# Asmodeus

**Modern Assembly Language Inspired by Machine W Architecture**

```
 ______                                  __                            
/\  _  \                                /\ \                           
\ \ \_\ \    ____    ___ ___     ___    \_\ \     __   __  __    ____  
 \ \  __ \  /',__\ /' __` __`\  / __`\  /'_` \  /'__`\/\ \/\ \  /',__\ 
  \ \ \/\ \/\__, `\/\ \/\ \/\ \/\ \_\ \/\ \_\ \/\  __/\ \ \_\ \/\__, `\
   \ \_\ \_\/\____/\ \_\ \_\ \_\ \____/\ \___,_\ \____\\ \____/\/\____/
    \/_/\/_/\/___/  \/_/\/_/\/_/\/___/  \/__,_ /\/____/ \/___/  \/___/ 
```

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

Asmodeus is a complete assembly language toolchain inspired by the legendary Machine W architecture. It features a full compiler pipeline from source code to executable machine code, with advanced debugging capabilities and an extensible instruction set.

## 🚀 Features

### Core Capabilities
- **Complete Toolchain**: Lexer → Parser → Assembler → Virtual Machine
- **Machine W Architecture**: 16-bit word size, 2048 words of memory
- **Extended Instruction Set**: Advanced arithmetic operations (MNO, DZI, MOD)
- **Interactive Debugger**: Bugseer - step-by-step execution with breakpoints
- **Real-time I/O**: Character-based input/output for interactive programs
- **Multiple Addressing Modes**: Direct, immediate, indirect, register-based

### Advanced Features
- **Macro System**: Define reusable code blocks with parameters
- **Label Resolution**: Forward and backward references
- **Error Diagnostics**: Comprehensive error reporting with line numbers
- **Binary Disassembly**: Convert machine code back to readable assembly
- **Verbose Debugging**: Detailed execution tracing and state inspection

## 📦 Installation

### Quick Install (Recommended)
```bash
curl -sSL https://raw.githubusercontent.com/szymonwilczek/asmodeus/main/installers/install.sh | bash
```

### Manual Installation
1. Clone the repository:
```bash
git clone https://github.com/szymonwilczek/asmodeus.git
cd asmodeus
```

2. Build and install:
```bash
cargo build --release
cp target/release/asmodeus ~/.local/bin/asmod
```

3. Add to PATH (add to ~/.bashrc or ~/.zshrc):
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Development Setup
```bash
git clone https://github.com/szymonwilczek/asmodeus.git
cd asmodeus
cargo build
# Use cargo run -- [args] or ./installers/dev.sh [args] for development
```

## 🎯 Quick Start

### Hello World
Create `hello.asmod`:
```assembly
; Simple hello world program
start:
    POB message     ; Load message value into accumulator
    WYJSCIE         ; Output the value
    STP             ; Stop program

message: RST 42     ; Our "hello world" message (42)
```

Run it:
```bash
asmod run hello.asmod
```

### Basic Arithmetic
```assembly
; Add two numbers
start:
    POB first       ; Load first number
    DOD second      ; Add second number  
    WYJSCIE         ; Output result
    STP             ; Stop

first:  RST 25      ; First operand
second: RST 17      ; Second operand
```

### Extended Instruction Set Example
```assembly
; Calculate (15 * 3) / 5 = 9
start:
    POB #15         ; Load immediate value 15
    MNO #3          ; Multiply by 3 (extended instruction)
    DZI #5          ; Divide by 5 (extended instruction)
    WYJSCIE         ; Output result (9)
    STP

; Run with: asmod run --extended program.asmod
```

## 🛠️ Usage

### Command Line Interface

```bash
# Run assembly program (default mode)
asmod run program.asmod
asmod program.asmod                    # Same as above

# Assemble to binary without running
asmod assemble program.asmod -o program.bin

# Disassemble binary back to assembly
asmod disassemble program.bin

# Interactive debugger with breakpoints
asmod debug program.asmod

# Real-time character I/O mode
asmod interactive program.asmod

# Enable extended instruction set
asmod run --extended program.asmod

# Verbose output for debugging
asmod run --verbose --debug program.asmod
```

### Options
- `-o, --output FILE`: Specify output file
- `-v, --verbose`: Verbose output during compilation and execution
- `--debug`: Enable debug output (tokens, AST, etc.)
- `-e, --extended`: Enable extended instruction set (MNO, DZI, MOD)
- `-h, --help`: Show help message

## 📚 Language Reference

### Machine W Architecture

Asmodeus emulates the Machine W architecture with:
- **Memory**: 2048 words of 16-bit memory (addresses 0-2047)
- **Registers**:
  - `AK` - Accumulator (16-bit)
  - `L` - Instruction counter (11-bit, 0-2047) 
  - `AD` - Address register (11-bit)
  - `KOD` - Opcode register (5-bit)
  - `WS` - Stack pointer (11-bit, grows downward from 2047)
  - `R0-R7` - General purpose registers (16-bit each)

### Core Instruction Set

#### Arithmetic Instructions
- `DOD addr` - Add memory[addr] to AK
- `ODE addr` - Subtract memory[addr] from AK
- `DOD #value` - Add immediate value to AK
- `ODE #value` - Subtract immediate value from AK

#### Memory Instructions  
- `ŁAD addr` / `LAD addr` - Store AK to memory[addr]
- `POB addr` - Load memory[addr] to AK
- `POB #value` - Load immediate value to AK

#### Control Flow Instructions
- `SOB addr` - Unconditional jump to addr
- `SOM addr` - Jump to addr if AK < 0
- `SOZ addr` - Jump to addr if AK = 0
- `STP` - Stop program execution

#### Stack Instructions
- `SDP` - Push AK to stack
- `PZS` - Pop from stack to AK

#### I/O Instructions
- `WEJSCIE` / `WPR` - Read input to AK
- `WYJSCIE` / `WYJ` - Output AK value

#### Interrupt Instructions
- `DNS` - Disable interrupts
- `CZM` - Clear interrupt mask
- `MSK` - Set interrupt mask
- `PWR` - Return from interrupt

### Extended Instruction Set

Enable with `--extended` flag:
- `MNO addr` - Multiply AK by memory[addr]
- `DZI addr` - Divide AK by memory[addr]  
- `MOD addr` - AK = AK % memory[addr]
- `MNO #value` - Multiply AK by immediate value
- `DZI #value` - Divide AK by immediate value
- `MOD #value` - AK = AK % immediate value

### Addressing Modes

- **Direct**: `POB 100` - Use memory[100]
- **Immediate**: `POB #42` - Use literal value 42
- **Indirect**: `POB [100]` - Use memory[memory[100]]
- **Register**: `POB R1` - Use register R1 value
- **Register Indirect**: `POB [R1]` - Use memory[R1]

### Directives

- `RST value` - Reserve memory and initialize with value
- `RPA` - Reserve memory without initialization (0)

### Macros

```assembly
MAKRO macro_name param1 param2
    ; macro body
    DOD param1
    SOB param2
KONM

; Usage
start:
    macro_name 100 end_label
```

### Labels and Comments

```assembly
; Line comment
// C-style comment

label_name:     ; Define label
    POB data    ; Reference label
    SOB label_name

data: RST 42
```

## 🐛 Debugging with Bugseer

Asmodeus includes **Bugseer**, a powerful interactive debugger:

```bash
asmod debug program.asmod
```

### Debugger Commands
- `s` / `step` - Execute single instruction
- `c` / `continue` - Continue execution until breakpoint or end
- `d` / `display` - Show current machine state
- `b ADDRESS` / `breakpoint ADDRESS` - Set breakpoint at address
- `rb ADDRESS` - Remove breakpoint
- `lb` - List all breakpoints
- `m START [END]` - Dump memory range
- `h` / `help` - Show all commands
- `q` / `quit` - Exit debugger

### Example Debug Session
```
(bugseer)> b 5          # Set breakpoint at address 5
(bugseer)> c            # Continue until breakpoint
(bugseer)> d            # Display machine state
(bugseer)> m 0 10       # Show memory 0-10
(bugseer)> s            # Step one instruction
```

## 📁 Project Structure

The Asmodeus toolchain consists of several interconnected crates:

```
asmodeus/
├── src/                    # Main CLI application
├── lexariel/              # Lexical analyzer (tokenizer)
├── parseid/               # Parser (tokens → AST)
├── hephasm/               # Assembler (AST → machine code)
├── asmachina/             # Virtual machine (Machine W emulator)
├── dismael/               # Disassembler (machine code → assembly)
├── shared/                # Shared types and utilities
├── examples/              # Example programs
│   ├── basic/            # Simple examples
│   ├── arithmetic/       # Math operations
│   ├── extended_set/     # Extended instruction examples
│   ├── io/               # Input/output examples
│   └── errors/           # Error demonstration
└── tests/                # Integration tests
```

### Pipeline Flow
```
Source Code (.asmod)
    ↓ [Lexariel]
Tokens
    ↓ [Parseid] 
Abstract Syntax Tree (AST)
    ↓ [Hephasm]
Machine Code
    ↓ [Asmachina]
Execution Results
```

## 📖 Examples

### Factorial Calculation
```assembly
; Calculate 5! = 120
start:
    POB one         ; result = 1
    LAD result
    POB n           ; counter = 5  
    LAD counter

loop:
    POB counter     ; if counter == 0, done
    SOZ done
    
    POB result      ; result *= counter
    MNO counter     ; (requires --extended)
    LAD result
    
    POB counter     ; counter--
    ODE one
    LAD counter
    
    SOB loop

done:
    POB result      ; output result
    WYJSCIE
    STP

n:       RST 5
one:     RST 1  
result:  RPA
counter: RPA
```

### Character I/O Program  
```assembly
; Echo program with real-time I/O
start:
    WEJSCIE         ; Read character
    WYJSCIE         ; Echo it back
    STP

; Run with: asmod interactive echo.asmod
```

### Stack Operations
```assembly
; Demonstrate stack usage
start:
    POB #10         ; Push 10 to stack
    SDP
    POB #20         ; Push 20 to stack  
    SDP
    
    PZS             ; Pop 20 to AK
    WYJSCIE         ; Output 20
    PZS             ; Pop 10 to AK
    WYJSCIE         ; Output 10
    STP
```

For more examples, checkout [examples directory](/examples/)

## 🔧 Development

### Building from Source
```bash
git clone https://github.com/szymonwilczek/asmodeus.git
cd asmodeus
cargo build --release
```

### Running Tests
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Test specific crate
cargo test -p lexariel
cargo test -p asmachina
```

### Development Commands
```bash
# Use development wrapper
./installers/dev.sh run examples/basic/hello.asmod

# Or use cargo directly
cargo run -- run examples/basic/hello.asmod
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Structure Guidelines
- `lexariel/` - Lexical analysis and tokenization
- `parseid/` - Syntax analysis and AST generation  
- `hephasm/` - Assembly and code generation
- `asmachina/` - Virtual machine and execution
- `dismael/` - Disassembly and reverse engineering
- `shared/` - Common types and utilities

## 🐛 Troubleshooting

### Common Issues

**Extended instructions not working**
```bash
# Make sure to use --extended flag
asmod run --extended program.asmod
```

**File extension errors**
```bash
# Use .asmod for source files
asmod run program.asmod

# Use .bin for binary files (warning: binary files should contain valid Asmodeus syntax!) 
asmod disassemble program.bin
```

**Division by zero error**
```bash
# Check for DZI or MOD with zero operand
DZI #0  ; This will cause runtime error
```

**Undefined symbol error**
```bash
# Make sure all labels are defined
POB undefined_label  ; Error: symbol not found
```

### Getting Help

```bash
asmod --help                    # Show all options
asmod debug program.asmod       # Use interactive debugger
asmod run --verbose program.asmod  # Verbose output
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## 🙏 Acknowledgments

- Inspired by the classic Machine W architecture
- Built with modern Rust for performance and safety
- Designed for educational purposes and assembly language learning

## 🔗 Links

- [Repository](https://github.com/szymonwilczek/asmodeus)
- [Issues](https://github.com/szymonwilczek/asmodeus/issues)

---

**Made with ❤️ for assembly language enthusiasts**
