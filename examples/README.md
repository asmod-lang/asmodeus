# Asmodeus Examples

**Example Programs for Learning Asmodeus Language**

```
┌───────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  ███████╗██╗  ██╗ █████╗ ███╗   ███╗██████╗ ██╗     ███████╗███████╗  │
│  ██╔════╝╚██╗██╔╝██╔══██╗████╗ ████║██╔══██╗██║     ██╔════╝██╔════╝  │
│  █████╗   ╚███╔╝ ███████║██╔████╔██║██████╔╝██║     █████╗  ███████╗  │
│  ██╔══╝   ██╔██╗ ██╔══██║██║╚██╔╝██║██╔═══╝ ██║     ██╔══╝  ╚════██║  │
│  ███████╗██╔╝ ██╗██║  ██║██║ ╚═╝ ██║██║     ███████╗███████╗███████║  │
│  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝     ╚══════╝╚══════╝╚══════╝  │
│             Comprehensive Examples for Asmodeus Language              │
└───────────────────────────────────────────────────────────────────────┘
```

This directory contains a comprehensive collection of example programs written in Asmodeus language. These examples demonstrate various features of the language, from basic operations to advanced programming techniques including macros, extended instruction sets, and complex algorithms.

## 📁 Directory Structure

```
examples/
├── basic/              # Simple introductory programs
├── arithmetic/         # Mathematical operations and algorithms
├── extended_set/       # Examples using extended instruction set
├── io/                 # Input/output operations
├── advanced/           # Complex programs with macros and advanced features
├── arrays/             # Array manipulation examples
└── errors/             # Programs demonstrating error conditions
```

## 🚀 Getting Started

### Running Examples

```bash
# Basic hello world
asmod run examples/basic/hello.asmod

# Extended arithmetic (requires --extended flag)
asmod run --extended examples/extended_set/mno.asmod

# Interactive I/O program
asmod interactive examples/io/echo.asmod

# Debug mode for learning
asmod debug examples/basic/countdown.asmod
```

### Understanding the Output

Most examples produce numerical output representing their results:
- **42** - Common test value (represents "hello world" in our language)
- **Calculations** - Mathematical results from arithmetic operations
- **Character codes** - ASCII values for text-based programs

## 📚 Example Categories

### 🎯 Basic Examples (`basic/`)

Perfect for beginners learning Asmodeus assembly.

#### [`hello.asmod`](basic/hello.asmod)
```assembly
; Simple hello world program for Machine W
start:
    POB message     ; Load message value into accumulator
    WYJSCIE         ; Output the value
    STP             ; Stop program

message: RST 42     ; Our "hello world" message
```
**Run:** `asmod run examples/basic/hello.asmod`
**Output:** `42` (represents hello world)


### 🧮 Arithmetic Examples (`arithmetic/`)

Mathematical operations and algorithms.

#### [`dod.asmod`](arithmetic/dod.asmod)
```assembly
; Simple addition example
start:
    POB first       ; Load first number
    DOD second      ; Add second number
    WYJSCIE         ; Output sum
    STP             ; Stop

first:  RST 25      ; First addend
second: RST 17      ; Second addend
```
**Run:** `asmod run examples/arithmetic/dod.asmod`
**Output:** `42` (25 + 17)
**Demonstrates:** Basic addition, immediate vs direct addressing

### ⚡ Extended Set Examples (`extended_set/`)

Programs using extended arithmetic instructions (require `--extended` flag).

#### [`mno.asmod`](extended_set/mno.asmod)
```assembly
; Multiplication using extended instruction
start:
    POB #10      ; AK = 10
    MNO #5       ; Multiply by 5 (50)
    WYJSCIE      ; Output result
    STP
```
**Run:** `asmod run --extended examples/extended_set/mno.asmod`
**Output:** `50` (10 × 5)
**Demonstrates:** Extended multiplication instruction

### 🔧 Advanced Examples (`advanced/`)

Complex programs using macros and advanced language features.

#### [`macro.asmod`](advanced/macro.asmod)
```assembly
; Demonstration of macro system
MAKRO add_and_output num1 num2
    POB num1        ; Load first parameter
    DOD num2        ; Add second parameter
    WYJSCIE         ; Output result
KONM

MAKRO multiply_by_two value
    POB value       ; Load parameter
    DOD value       ; Add to itself (multiply by 2)
KONM

start:
    add_and_output #10 #20      ; Should output 30
    multiply_by_two #21         ; Should output 42
    STP
```
**Run:** `asmod run examples/advanced/macro.asmod`
**Output:** `30, 42`
**Demonstrates:** Macro definition, macro calls with parameters, code reuse

### 📊 Array Examples (`arrays/`)

Array manipulation and data structure examples.

#### [`array.asmod`](arrays/array.asmod)
```assembly
; Basic array access and manipulation
start:
    POB array_base  ; Load base address
    DOD offset      ; Add offset for array[2]
    LAD temp_addr   ; Store calculated address
    
    POB [temp_addr] ; Load value from calculated address
    WYJSCIE         ; Output array[2]
    STP

array_base: RST array_data
offset:     RST 2           ; Access third element (0-indexed)
temp_addr:  RPA

array_data:
    RST 10      ; array[0]
    RST 20      ; array[1] 
    RST 30      ; array[2]
    RST 40      ; array[3]
```
**Run:** `asmod run examples/arrays/array.asmod`
**Output:** `30` (array[2])
**Demonstrates:** Array indexing, indirect addressing, address calculation

### ❌ Error Examples (`errors/`)

Programs that demonstrate various error conditions for learning purposes.

#### [`error_lexer.asmod`](errors/error_lexer.asmod)
```assembly
; File with lexical error
; Contains invalid token that will cause lexer error
start:
    POB @invalid    ; @ is not a valid character
    STP
```
**Run:** `asmod run examples/errors/error_lexer.asmod`
**Result:** Lexer error: "Invalid character '@' at line x, column y"
**Demonstrates:** Lexical analysis errors

#### [`error_parser.asmod`](errors/error_parser.asmod)
```assembly
; File with parser error
; Missing operand for instruction
start:
    POB             ; Missing operand - parser error
    STP
```
**Run:** `asmod run examples/errors/error_parser.asmod`
**Result:** Parser error: "Expected operand, found 'STP' at line x"
**Demonstrates:** Syntax errors

#### [`error_assembler.asmod`](errors/error_assembler.asmod)
```assembly
; File with assembler error
; References undefined symbol
start:
    POB undefined_symbol    ; This symbol is not defined anywhere
    STP
```
**Run:** `asmod run examples/errors/error_assembler.asmod`
**Result:** Assembler error: "Undefined symbol 'undefined_symbol' at line x"
**Demonstrates:** Symbol resolution errors

## 🎓 Learning Path

### For Beginners
1. **Start with basic examples**
   - [`hello.asmod`](basic/hello.asmod) - Understanding basic structure
   - [`test.asmod`](basic/test.asmod) - Memory operations
   - [`countdown.asmod`](basic/countdown.asmod) - Loops and control flow

2. **Learn arithmetic operations**
   - [`dod.asmod`](arithmetic/dod.asmod) - Simple addition
   - [`nnw.asmod`](arithmetic/nnw.asmod) - Algorithm implementation

3. **Explore I/O operations**
   - [`echo.asmod`](io/echo.asmod) - Interactive input/output

### For Intermediate Users
1. **Extended instruction set**
   - [`mno.asmod`](extended_set/mno.asmod) - Multiplication
   - [`div.asmod`](extended_set/div.asmod) - Division
   - [`complex_calc.asmod`](extended_set/complex_calc.asmod) - Complex calculations

2. **Advanced programming**
   - [`macro.asmod`](advanced/macro.asmod) - Macro system
   - [`factorial.asmod`](arithmetic/factorial.asmod) - Complex algorithms

3. **Data structures**
   - [`array.asmod`](arrays/array.asmod) - Array basics
   - [`max_element_array.asmod`](arrays/max_element_array.asmod) - Array algorithms

### For Advanced Users
1. **Error handling and debugging**
   - Study all examples in [`errors/`](errors/) directory
   - Use `asmod debug` with complex examples

2. **Program optimization**
   - Compare different implementation approaches
   - Analyze generated machine code with `asmod disassemble`

## 🛠️ Testing Examples

### Disassembly Analysis
```bash
# See how source compiles to machine code
asmod assemble examples/basic/hello.asmod -o hello.bin
asmod disassemble hello.bin
```

## 📝 Creating Your Own Examples

### Example Template
```assembly
; [Description of what this program does]
; [Any special requirements or notes]

start:
    ; Your code here
    POB data_value
    WYJSCIE
    STP

; Data section
data_value: RST 42
```

### Best Practices
1. **Add comments** explaining the purpose and logic
2. **Use meaningful labels** for data and code sections
3. **Include expected output** in comments
4. **Test thoroughly** before adding to examples
5. **Follow naming conventions** used in existing examples

### Contributing Examples
1. Create your program in the appropriate subdirectory
2. Test it with `asmod run your_example.asmod`
3. Add documentation explaining what it demonstrates
4. Include expected output and any special requirements

## 🔗 Related Documentation

- **[Asmodeus README](../README.md)** - Complete language overview
- **[Asmachina](../asmachina/README.md)** - Virtual machine documentation
- **[Language Reference](../docs/language_reference.md)** - Complete syntax guide
- **[Installation Guide](../docs/installation.md)** - Setup instructions

## 💡 Tips for Learning

1. **Start Simple**: Begin with basic examples and gradually work up to complex ones
2. **Use Debug Mode**: `asmod debug` lets you step through execution
3. **Experiment**: Modify examples to see how changes affect behavior
4. **Read Comments**: Examples include detailed explanations
5. **Compare Approaches**: Look at different ways to solve similar problems

---

**Happy Learning with Asmodeus Assembly! 🎯**

*These examples represent the journey from simple concepts to advanced programming techniques in Asmodeus language. Each program is designed to teach specific concepts while building towards more complex programming skills.*
