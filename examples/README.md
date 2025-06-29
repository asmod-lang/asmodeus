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

#### [`test.asmod`](basic/test.asmod)
```assembly
; Basic load and store operations
start:
    POB value       ; Load value from memory
    ŁAD result      ; Store to result location
    WYJSCIE         ; Output the result
    STP             ; Stop

value: RST 42       ; Input value
result: RPA         ; Output location
```
**Run:** `asmod run examples/basic/test.asmod`
**Demonstrates:** Memory operations, data definition

#### [`countdown.asmod`](basic/countdown.asmod)
```assembly
; Countdown from 5 to 1
start:
    POB counter     ; Load current counter value
    WYJSCIE         ; Output current number
    ODE one         ; Subtract 1
    ŁAD counter     ; Store back to counter
    SOM end         ; Jump to end if negative
    SOB start       ; Loop back to start
    
end:
    STP             ; Stop program

counter: RST 5      ; Start counting from 5
one:     RST 1      ; Constant for decrement
```
**Run:** `asmod run examples/basic/countdown.asmod`
**Output:** `5, 4, 3, 2, 1`
**Demonstrates:** Loops, conditional jumps, counters

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

#### [`factorial.asmod`](arithmetic/factorial.asmod)
```assembly
; Calculate factorial of 5 (5! = 120)
; Complex iterative approach using multiplication
start:
    POB one         ; Load 1 into AK (initial result)
    LAD result      ; Store 1 as initial result
    POB n           ; Load n=5 into AK
    LAD counter     ; Store as counter

factorial_loop:
    POB counter     ; Load current counter
    SOZ done        ; If counter == 0, we're done
    
    ; Multiply result by counter using repeated addition
    LAD multiplier  ; Store counter as multiplier
    POB result      ; Load current result
    LAD temp_result ; Store as temporary
    
    POB zero        ; Clear result
    LAD result      
    
mult_loop:
    POB multiplier  ; Check multiplier
    SOZ mult_done   ; If 0, multiplication done
    
    POB result      ; Add temp_result to result
    DOD temp_result
    LAD result
    
    POB multiplier  ; Decrement multiplier
    ODE one
    LAD multiplier
    SOB mult_loop
    
mult_done:
    POB counter     ; Decrement main counter
    ODE one
    LAD counter
    SOB factorial_loop
    
done:
    POB result      ; Output final result
    WYJSCIE         ; Should output 120
    STP

; Data section
n:           RST 5       ; Calculate 5!
one:         RST 1       ; Constant 1
zero:        RST 0       ; Constant 0
result:      RST 1       ; Result accumulator
counter:     RPA         ; Loop counter
multiplier:  RPA         ; Multiplication counter
temp_result: RPA         ; Temporary storage
```
**Run:** `asmod run examples/arithmetic/factorial.asmod`
**Output:** `120` (5! = 5×4×3×2×1)
**Demonstrates:** Complex loops, multiplication by addition, algorithm implementation

#### [`nnw.asmod`](arithmetic/nnw.asmod)
```assembly
; Greatest Common Divisor (GCD) algorithm
; Euclidean algorithm implementation
POB a           ; Load first number
ŁAD a1          ; Store working copy
POB b           ; Load second number  
ŁAD b1          ; Store working copy

loop:
    POB a1      ; Load a1
    ODE b1      ; Subtract b1
    SOZ koniec  ; If zero, we found GCD
    
    POB b1      ; Check which is larger
    ODE a1
    SOM tak     ; If a1 > b1, jump
    
    POB a1      ; a1 is smaller, add original a
    DOD a
    ŁAD a1
    SOB loop
    
tak:            ; b1 is smaller, add original b
    POB b1
    DOD b
    ŁAD b1
    SOB loop
    
koniec:         ; Result is in a1
    POB a1
    WYJSCIE
    STP

; Test data
a:  RST 14      ; First number
b:  RST 12      ; Second number
a1: RPA         ; Working variable
b1: RPA         ; Working variable
```
**Run:** `asmod run examples/arithmetic/nnw.asmod`
**Output:** `2` (GCD of 14 and 12)
**Demonstrates:** Mathematical algorithms, complex control flow

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

#### [`div.asmod`](extended_set/div.asmod)
```assembly
; Division using extended instruction
start:
    POB #100     ; AK = 100 
    DZI #4       ; Divide by 4 (25)
    WYJSCIE      ; Output result
    STP
```
**Run:** `asmod run --extended examples/extended_set/div.asmod`
**Output:** `25` (100 ÷ 4)
**Demonstrates:** Extended division instruction

#### [`mod.asmod`](extended_set/mod.asmod)
```assembly
; Modulo operation using extended instruction
start:
    POB #17      ; AK = 17
    MOD #5       ; Modulo 5 (remainder: 2)
    WYJSCIE      ; Output result
    STP
```
**Run:** `asmod run --extended examples/extended_set/mod.asmod`
**Output:** `2` (17 % 5)
**Demonstrates:** Extended modulo instruction

#### [`complex_calc.asmod`](extended_set/complex_calc.asmod)
```assembly
; Complex calculation: (15 * 3) / 5 + 2, then 20 % 7
start:
    ; First calculation: (15 * 3) / 5 + 2
    POB #15      ; AK = 15
    MNO #3       ; AK = 15 * 3 = 45
    DZI #5       ; AK = 45 / 5 = 9
    DOD #2       ; AK = 9 + 2 = 11
    WYJSCIE      ; Output 11
    
    ; Second calculation: 20 % 7
    POB #20      ; AK = 20
    MOD #7       ; AK = 20 % 7 = 6
    WYJSCIE      ; Output 6
    
    STP
```
**Run:** `asmod run --extended examples/extended_set/complex_calc.asmod`
**Output:** `11, 6`
**Demonstrates:** Chaining extended operations, multiple calculations

#### [`div_zero.asmod`](extended_set/div_zero.asmod) - Error Example
```assembly
; Demonstrates division by zero error
start:
    POB #42      ; AK = 42
    DZI #0       ; Division by zero - should cause error!
    WYJSCIE      ; Won't reach here
    STP
```
**Run:** `asmod run --extended examples/extended_set/div_zero.asmod`
**Result:** Runtime error: "Division by zero at address 1"
**Demonstrates:** Error handling, runtime validation

#### [`no_extended_error.asmod`](extended_set/no_extended_error.asmod) - Error Example
```assembly
; Shows error when extended instructions used without --extended flag
start:
    POB #10      ; AK = 10
    MNO #5       ; Error: extended instruction without --extended flag
    WYJSCIE      ; Won't reach here
    STP
```
**Run:** `asmod run examples/extended_set/no_extended_error.asmod` (without --extended)
**Result:** Assembly error: "Extended instruction 'MNO' not enabled"
**Demonstrates:** Extended instruction validation

### 💾 I/O Examples (`io/`)

Input and output operations, including interactive programs.

#### [`echo.asmod`](io/echo.asmod)
```assembly
; Simple character echo program
; Reads a character and outputs it
start:
    WEJSCIE         ; Read character from input
    WYJSCIE         ; Output the character
    STP             ; Stop
```
**Run:** `asmod interactive examples/io/echo.asmod`
**Demonstrates:** Interactive I/O, character input/output

#### [`dollar.asmod`](io/dollar.asmod)
```assembly
; Count '$' characters in input
; Reads characters until count reaches threshold
start:
loop:   
    POB remaining   ; Load remaining characters to read
    ODE one         ; Subtract 1
    SOM end         ; If negative, we're done
    LAD remaining   ; Store back
    
    WEJSCIE         ; Read character
    ODE dollar      ; Subtract '$' ASCII value (36)
    SOZ found       ; If zero (found '$'), increment counter
    SOB loop        ; Continue loop
    
found:  
    POB count       ; Load counter
    DOD one         ; Increment it  
    LAD count       ; Store back
    SOB loop        ; Continue loop

end:    
    POB count       ; Load final count
    WYJSCIE         ; Output count as number
    STP             ; Stop

; Variables
count:      RST 0       ; Counter for '$' characters
dollar:     RST 36      ; ASCII value of '$'  
one:        RST 1       ; Constant 1
remaining:  RST 10      ; Number of characters to read
```
**Run:** `asmod interactive examples/io/dollar.asmod`
**Input:** Type characters including some '$' symbols
**Output:** Count of '$' characters found
**Demonstrates:** Character processing, counting algorithms, ASCII values

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

#### [`max_element_array.asmod`](arrays/max_element_array.asmod)
```assembly
; Find maximum element in array
start:
    POB array_start ; Load first element as initial max
    LAD max_value
    POB #1          ; Start from second element
    LAD index
    
loop:
    POB index       ; Check if we've processed all elements
    ODE array_size
    SOZ done        ; If index >= size, we're done
    
    ; Calculate address of current element
    POB array_start_addr
    DOD index
    LAD current_addr
    
    ; Load current element
    POB [current_addr]
    LAD current_value
    
    ; Compare with current max
    POB current_value
    ODE max_value
    SOM not_greater ; If current <= max, skip update
    
    ; Current is greater, update max
    POB current_value
    LAD max_value
    
not_greater:
    POB index       ; Increment index
    DOD #1
    LAD index
    SOB loop
    
done:
    POB max_value   ; Output maximum value
    WYJSCIE
    STP

; Data
array_start_addr: RST array_data
array_size:       RST 5
max_value:        RPA
index:            RPA
current_addr:     RPA
current_value:    RPA

array_data:
    RST 15      ; array[0]
    RST 7       ; array[1]
    RST 23      ; array[2] - maximum
    RST 11      ; array[3]
    RST 19      ; array[4]
```
**Run:** `asmod run examples/arrays/max_element_array.asmod`
**Output:** `23` (maximum element)
**Demonstrates:** Array traversal, comparison operations, algorithm implementation

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
