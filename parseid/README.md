# Parseid

**Parser for Asmodeus Language**

```
┌───────────────────────────────────────────────────────┐
│                                                       │
│  ██████╗  █████╗ ██████╗ ███████╗███████╗██╗██████╗   │
│  ██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝██║██╔══██╗  │
│  ██████╔╝███████║██████╔╝███████╗█████╗  ██║██║  ██║  │
│  ██╔═══╝ ██╔══██║██╔══██╗╚════██║██╔══╝  ██║██║  ██║  │
│  ██║     ██║  ██║██║  ██║███████║███████╗██║██████╔╝  │
│  ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═════╝   │
│                                                       │
│          Tokens Parser for Asmodeus Language          │
└───────────────────────────────────────────────────────┘
```

**Parseid** is the syntax analyzer for Asmodeus assembly language. It transforms the token stream from Lexariel into a structured Abstract Syntax Tree (AST) that represents the semantic structure of the program. Built with robust error handling and recovery capabilities.

## 🎯 Features

### Core Parsing Capabilities
- **Complete Syntax Analysis**: All Asmodeus language constructs
- **AST Generation**: Well-structured Abstract Syntax Tree output
- **Macro System**: Full macro definition and call parsing
- **Label Resolution**: Forward and backward label reference support
- **Directive Processing**: Assembler directives (`RST`, `RPA`, etc.)
- **Error Recovery**: Continues parsing after syntax errors

### Advanced Features
- **Multiple Addressing Modes**: Direct, immediate, indirect, register-based
- **Operand Validation**: Type checking and format validation
- **Position Tracking**: Maintains source location for error reporting
- **Instruction Validation**: Ensures correct instruction formats
- **Comment Preservation**: Maintains documentation in AST

## 🚀 Quick Start

### Basic Usage

```rust
use parseid::{parse, parse_source};
use lexariel::tokenize;

// Parse from source code directly
let source = r#"
    start:
        POB #42     ; Load immediate value
        WYJSCIE     ; Output the value  
        STP         ; Stop program
"#;

let ast = parse_source(source)?;
println!("Program has {} elements", ast.elements.len());

// Or parse from tokens
let tokens = tokenize(source)?;
let ast = parse(tokens)?;
```

### Examining the AST

```rust
use parseid::ast::*;

let source = r#"
    main:
        POB data_value
        DOD #10
        WYJSCIE
        STP
        
    data_value: RST 42
"#;

let ast = parse_source(source)?;

for element in ast.elements {
    match element {
        ProgramElement::LabelDefinition(label) => {
            println!("Label: {} at line {}", label.name, label.line);
        }
        ProgramElement::Instruction(instr) => {
            println!("Instruction: {} at line {}", instr.opcode, instr.line);
            if let Some(operand) = instr.operand {
                println!("  Operand: {} (mode: {:?})", 
                         operand.value, operand.addressing_mode);
            }
        }
        ProgramElement::Directive(dir) => {
            println!("Directive: {} with {} args", 
                     dir.name, dir.arguments.len());
        }
        _ => {}
    }
}
```

## 📚 AST Structure

### Program Elements

The AST represents a program as a collection of elements:

```rust
pub struct Program {
    pub elements: Vec<ProgramElement>,
}

pub enum ProgramElement {
    LabelDefinition(LabelDefinition),
    Instruction(Instruction),
    Directive(Directive),
    MacroDefinition(MacroDefinition),
    MacroCall(MacroCall),
}
```

### Instructions

```rust
pub struct Instruction {
    pub opcode: String,
    pub operand: Option<Operand>,
    pub line: usize,
    pub column: usize,
}

pub struct Operand {
    pub value: String,
    pub addressing_mode: AddressingMode,
}

pub enum AddressingMode {
    Direct,                    // POB value
    Immediate,                 // POB #42
    Indirect,                  // POB [value]
    MultipleIndirect,          // POB [[value]]
    Register,                  // POB R1
    RegisterIndirect,          // POB [R1]
    BaseRegister { base: u8, offset: u8 },  // POB R1+5
    Relative,                  // POB +10 or POB -5
    Indexed { base: String, index: String }, // POB array[index]
}
```

### Labels and Directives

```rust
pub struct LabelDefinition {
    pub name: String,
    pub line: usize,
    pub column: usize,
}

pub struct Directive {
    pub name: String,           // "RST", "RPA", etc.
    pub arguments: Vec<String>, // Arguments to directive
    pub line: usize,
    pub column: usize,
}
```

### Macros

```rust
pub struct MacroDefinition {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<ProgramElement>,
    pub line: usize,
    pub column: usize,
}

pub struct MacroCall {
    pub name: String,
    pub arguments: Vec<String>,
    pub line: usize,
    pub column: usize,
}
```

## 🔧 API Reference

### Main Functions

```rust
// Parse from source code (convenience function)
pub fn parse_source(source: &str) -> Result<Program, ParserError>;

// Parse from token vector
pub fn parse(tokens: Vec<Token>) -> Result<Program, ParserError>;
```

### Parser Class

For more control over parsing:

```rust
use parseid::Parser;

let tokens = tokenize(source)?;
let mut parser = Parser::new(tokens);
let ast = parser.parse()?;

// Access parser state if needed
println!("Parsing completed at token {}", parser.current_position());
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token '{token}' at line {line}, column {column}")]
    UnexpectedToken { token: String, line: usize, column: usize },
    
    #[error("Expected {expected}, found '{found}' at line {line}")]
    ExpectedToken { expected: String, found: String, line: usize },
    
    #[error("Invalid operand format '{operand}' at line {line}")]
    InvalidOperand { operand: String, line: usize },
    
    #[error("Unterminated macro definition starting at line {line}")]
    UnterminatedMacro { line: usize },
    
    #[error("Lexer error: {0}")]
    LexerError(#[from] lexariel::LexerError),
}
```

## 📖 Examples

### Basic Program Parsing

```rust
use parseid::{parse_source, ast::*};

let program = r#"
    ; Calculate sum of two numbers
    start:
        POB first       ; Load first number
        DOD second      ; Add second number
        WYJSCIE         ; Output result
        STP             ; Stop
        
    first:  RST 25      ; First operand
    second: RST 17      ; Second operand
"#;

let ast = parse_source(program)?;

// Count different element types
let mut instruction_count = 0;
let mut label_count = 0;
let mut directive_count = 0;

for element in &ast.elements {
    match element {
        ProgramElement::Instruction(_) => instruction_count += 1,
        ProgramElement::LabelDefinition(_) => label_count += 1,
        ProgramElement::Directive(_) => directive_count += 1,
        _ => {}
    }
}

println!("Instructions: {}, Labels: {}, Directives: {}", 
         instruction_count, label_count, directive_count);
```

### Addressing Mode Analysis

```rust
let addressing_examples = r#"
    test_addressing:
        POB value        ; Direct
        POB #42          ; Immediate
        POB [address]    ; Indirect
        POB [[ptr]]      ; Multiple indirect
        POB R1           ; Register
        POB [R1]         ; Register indirect
        POB R1+5         ; Base + offset
        POB +10          ; Relative positive
        POB -5           ; Relative negative
        STP
"#;

let ast = parse_source(addressing_examples)?;

for element in ast.elements {
    if let ProgramElement::Instruction(instr) = element {
        if let Some(operand) = instr.operand {
            println!("Instruction {} uses {:?} addressing with '{}'",
                     instr.opcode, operand.addressing_mode, operand.value);
        }
    }
}
```

### Macro Definition Parsing

```rust
let macro_program = r#"
    MAKRO add_and_output num1 num2
        POB num1
        DOD num2
        WYJSCIE
    KONM
    
    MAKRO multiply_by_two value
        POB value
        DOD value
    KONM
    
    start:
        add_and_output #10 #20
        multiply_by_two result
        STP
        
    result: RPA
"#;

let ast = parse_source(macro_program)?;

// Extract macro definitions
let macros: Vec<_> = ast.elements.iter()
    .filter_map(|e| {
        if let ProgramElement::MacroDefinition(m) = e {
            Some(m)
        } else {
            None
        }
    })
    .collect();

for macro_def in macros {
    println!("Macro '{}' with parameters: {:?}", 
             macro_def.name, macro_def.parameters);
    println!("  Body has {} elements", macro_def.body.len());
}
```

### Directive Processing

```rust
let data_section = r#"
    ; Data definitions
    number:     RST 42          ; Initialize with value
    buffer:     RPA             ; Reserve without init
    array:      RST 1, 2, 3, 4  ; Array initialization
    string_len: RST 0x0D        ; Hex value
    flag:       RST 0b1010      ; Binary value
"#;

let ast = parse_source(data_section)?;

for element in ast.elements {
    if let ProgramElement::Directive(dir) = element {
        println!("Directive '{}' at line {} with args: {:?}",
                 dir.name, dir.line, dir.arguments);
    }
}
```

### Error Handling and Recovery

```rust
use parseid::{parse_source, ParserError};

let source_with_errors = r#"
    start:
        POB             ; Missing operand - error
        DOD #42         ; Valid instruction
        INVALID_OP      ; Invalid opcode - error  
        STP             ; Valid instruction
"#;

match parse_source(source_with_errors) {
    Ok(ast) => {
        println!("Parsed successfully with {} elements", ast.elements.len());
    }
    Err(ParserError::UnexpectedToken { token, line, column }) => {
        println!("Parse error: unexpected '{}' at {}:{}", token, line, column);
    }
    Err(ParserError::ExpectedToken { expected, found, line }) => {
        println!("Parse error: expected {} but found '{}' at line {}", 
                 expected, found, line);
    }
    Err(e) => {
        println!("Other parse error: {}", e);
    }
}
```

### Complex Program Analysis

```rust
let complex_program = r#"
    ; Factorial calculation using macros
    MAKRO factorial_step current result
        POB result
        MNO current     ; Requires extended mode
        LAD result
        
        POB current
        ODE one
        LAD current
    KONM
    
    start:
        POB n           ; Load initial value
        LAD current     ; Set counter
        POB one         ; Initialize result
        LAD result
        
    factorial_loop:
        POB current     ; Check if done
        SOZ done
        
        factorial_step current result
        SOB factorial_loop
        
    done:
        POB result      ; Output final result
        WYJSCIE
        STP
        
    ; Data section
    n:       RST 5      ; Calculate 5!
    one:     RST 1      ; Constant
    current: RPA        ; Current counter
    result:  RPA        ; Accumulator
"#;

let ast = parse_source(complex_program)?;

// Analyze program structure
let mut analysis = std::collections::HashMap::new();
for element in &ast.elements {
    let key = match element {
        ProgramElement::LabelDefinition(_) => "labels",
        ProgramElement::Instruction(i) => {
            match i.opcode.as_str() {
                "POB" | "DOD" | "ODE" | "MNO" => "arithmetic",
                "SOB" | "SOM" | "SOZ" => "control_flow", 
                "LAD" => "memory",
                "STP" => "control",
                _ => "other_instructions"
            }
        }
        ProgramElement::Directive(_) => "directives",
        ProgramElement::MacroDefinition(_) => "macro_definitions",
        ProgramElement::MacroCall(_) => "macro_calls",
    };
    
    *analysis.entry(key).or_insert(0) += 1;
}

println!("Program analysis: {:?}", analysis);
```

## 🧪 Testing

### Unit Tests

```bash
cargo test -p parseid
```

### Specific Test Categories

```bash
# Test instruction parsing
cargo test -p parseid instruction_tests

# Test addressing mode parsing
cargo test -p parseid addressing_tests

# Test macro parsing
cargo test -p parseid macro_tests

# Test directive parsing
cargo test -p parseid directive_tests

# Test error handling
cargo test -p parseid error_tests
```

### Integration Tests

```bash
cargo test -p parseid --test integration_tests
```

## 🔍 Performance Characteristics

- **Speed**: ~500K tokens per second parsing
- **Memory**: O(n) where n is number of tokens
- **Error Recovery**: Minimal performance impact
- **AST Size**: Typically 2-3x larger than token count

### Performance Testing

```rust
use parseid::parse_source;
use std::time::Instant;

let large_program = include_str!("large_program.asmod");
let start = Instant::now();
let ast = parse_source(large_program)?;
let duration = start.elapsed();

println!("Parsed {} elements in {:?}", ast.elements.len(), duration);
```

## 🔧 Advanced Usage

### Custom AST Processing

```rust
use parseid::ast::*;

struct AstVisitor {
    instruction_count: usize,
    max_operand_length: usize,
}

impl AstVisitor {
    fn visit_program(&mut self, program: &Program) {
        for element in &program.elements {
            self.visit_element(element);
        }
    }
    
    fn visit_element(&mut self, element: &ProgramElement) {
        match element {
            ProgramElement::Instruction(instr) => {
                self.instruction_count += 1;
                if let Some(operand) = &instr.operand {
                    self.max_operand_length = 
                        self.max_operand_length.max(operand.value.len());
                }
            }
            ProgramElement::MacroDefinition(macro_def) => {
                for body_element in &macro_def.body {
                    self.visit_element(body_element);
                }
            }
            _ => {}
        }
    }
}

let ast = parse_source(source)?;
let mut visitor = AstVisitor { 
    instruction_count: 0, 
    max_operand_length: 0 
};
visitor.visit_program(&ast);

println!("Found {} instructions, max operand length: {}", 
         visitor.instruction_count, visitor.max_operand_length);
```

### AST Transformation

```rust
use parseid::ast::*;

fn optimize_immediate_loads(mut program: Program) -> Program {
    for element in &mut program.elements {
        if let ProgramElement::Instruction(ref mut instr) = element {
            if instr.opcode == "POB" {
                if let Some(ref mut operand) = instr.operand {
                    // Convert small direct addresses to immediate if beneficial
                    if let AddressingMode::Direct = operand.addressing_mode {
                        if let Ok(addr) = operand.value.parse::<u16>() {
                            if addr < 256 {
                                operand.addressing_mode = AddressingMode::Immediate;
                                operand.value = format!("#{}", addr);
                            }
                        }
                    }
                }
            }
        }
    }
    program
}
```

## 🚫 Error Recovery

Parseid implements sophisticated error recovery:

```rust
let source_with_multiple_errors = r#"
    start:
        POB         ; Error: missing operand
        DOD #42     ; Valid after error
        BADOP       ; Error: invalid opcode
        WYJSCIE     ; Valid after error
        STP         ; Valid
"#;

// Parser will try to recover and continue parsing
match parse_source(source_with_multiple_errors) {
    Ok(ast) => {
        println!("Recovered and parsed {} elements", ast.elements.len());
        // AST will contain the valid instructions
    }
    Err(first_error) => {
        println!("First error: {}", first_error);
        // In advanced usage, could collect all errors
    }
}
```

## 🔗 Integration with Asmodeus Pipeline

Parseid sits between Lexariel and Hephasm in the pipeline:

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Lexariel  │───▶│   Parseid   │───▶│   Hephasm   │
│   (Lexer)   │    │  (Parser)   │    │ (Assembler) │
│             │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘
        │                   │                   │
        ▼                   ▼                   ▼
   ┌─────────┐         ┌─────────┐         ┌─────────┐
   │ Tokens  │         │   AST   │         │ Machine │
   │         │         │         │         │  Code   │
   └─────────┘         └─────────┘         └─────────┘
```

### Complete Pipeline Usage

```rust
use lexariel::tokenize;
use parseid::parse;
use hephasm::assemble_program;

let source = "POB #42\nWYJSCIE\nSTP";
let tokens = tokenize(source)?;      // Lexariel
let ast = parse(tokens)?;            // Parseid  
let machine_code = assemble_program(&ast)?; // Hephasm
```

## 🎨 AST Visualization

For debugging and development:

```rust
use parseid::ast::*;

fn print_ast_tree(program: &Program, indent: usize) {
    for element in &program.elements {
        print_element(element, indent);
    }
}

fn print_element(element: &ProgramElement, indent: usize) {
    let spaces = " ".repeat(indent);
    match element {
        ProgramElement::LabelDefinition(label) => {
            println!("{}Label: {}", spaces, label.name);
        }
        ProgramElement::Instruction(instr) => {
            println!("{}Instruction: {}", spaces, instr.opcode);
            if let Some(operand) = &instr.operand {
                println!("{}  Operand: {} ({:?})", 
                         spaces, operand.value, operand.addressing_mode);
            }
        }
        ProgramElement::MacroDefinition(macro_def) => {
            println!("{}Macro: {} {:?}", spaces, macro_def.name, macro_def.parameters);
            for body_element in &macro_def.body {
                print_element(body_element, indent + 2);
            }
        }
        _ => {}
    }
}
```

## 📜 License

This crate is part of the Asmodeus project and is licensed under the MIT License.

## 🔗 Related Components

- **[Lexariel](../lexariel/)** - Lexer that generates tokens for Parseid
- **[Hephasm](../hephasm/)** - Assembler that consumes Parseid AST
- **[Shared](../shared/)** - Common types and utilities
- **[Main Asmodeus](../)** - Complete language toolchain
