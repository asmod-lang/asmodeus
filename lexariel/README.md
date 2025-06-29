# Lexariel

**Lexical Analyzer for Asmodeus Language**

```
┌───────────────────────────────────────────────────────────────┐
│                                                               │
│  ██╗     ███████╗██╗  ██╗ █████╗ ██████╗ ██╗███████╗██╗       │
│  ██║     ██╔════╝╚██╗██╔╝██╔══██╗██╔══██╗██║██╔════╝██║       │
│  ██║     █████╗   ╚███╔╝ ███████║██████╔╝██║█████╗  ██║       │
│  ██║     ██╔══╝   ██╔██╗ ██╔══██║██╔══██╗██║██╔══╝  ██║       │
│  ███████╗███████╗██╔╝ ██╗██║  ██║██║  ██║██║███████╗███████╗  │
│  ╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝╚══════╝  │
│                                                               │
│                Asmodeus Language Tokenizer                    │
└───────────────────────────────────────────────────────────────┘
```

**Lexariel** is the lexical analyzer (tokenizer) for the Asmodeus language. It converts raw source code into a stream of structured tokens that can be consumed by the parser. Built with performance and error recovery in mind.

## 🎯 Features

### Core Tokenization
- **Complete Token Recognition**: All Asmodeus language constructs
- **Multiple Comment Styles**: Both `;` and `//` comment syntax
- **Number Format Support**: Decimal, hexadecimal (0x), and binary (0b)
- **Identifier Recognition**: Labels, instruction mnemonics, and symbols
- **Addressing Mode Tokens**: `#`, `[`, `]`, register names
- **Error Recovery**: Continues parsing after lexical errors

### Advanced Features
- **Position Tracking**: Line and column information for all tokens
- **Whitespace Handling**: Intelligent whitespace skipping
- **String Literals**: Support for quoted strings and character literals
- **Macro Keywords**: Recognition of `MAKRO`, `KONM` macro delimiters
- **Directive Support**: `RST`, `RPA` and other assembler directives

## 🚀 Quick Start

### Basic Usage

```rust
use lexariel::tokenize;

let source = r#"
    start:
        POB #42     ; Load immediate value
        WYJSCIE     // Output the value
        STP         ; Stop program
"#;

let tokens = tokenize(source)?;
for token in tokens {
    println!("{:?}", token);
}
```

### Output Example
```
Token { kind: Identifier, value: "start", line: 2, column: 5 }
Token { kind: Colon, value: ":", line: 2, column: 10 }
Token { kind: Keyword, value: "POB", line: 3, column: 9 }
Token { kind: Hash, value: "#", line: 3, column: 13 }
Token { kind: Number, value: "42", line: 3, column: 14 }
Token { kind: Keyword, value: "WYJSCIE", line: 4, column: 9 }
Token { kind: Keyword, value: "STP", line: 5, column: 9 }
```

### Advanced Tokenization

```rust
use lexariel::{Lexer, TokenKind};

let source = r#"
    MAKRO add_numbers num1 num2
        POB num1
        DOD num2
        WYJSCIE
    KONM
    
    data_section:
        value1: RST 0x2A    ; Hex number
        value2: RST 0b101010 ; Binary number
        buffer: RPA
"#;

let tokens = tokenize(source)?;

// Filter only keywords
let keywords: Vec<_> = tokens.iter()
    .filter(|t| t.kind == TokenKind::Keyword)
    .collect();

// Count different token types
let mut counts = std::collections::HashMap::new();
for token in &tokens {
    *counts.entry(token.kind).or_insert(0) += 1;
}
```

## 📚 Token Types

### Core Token Types

| Token Kind | Description | Examples |
|------------|-------------|----------|
| `Keyword` | Assembly instructions and directives | `POB`, `DOD`, `STP`, `RST` |
| `Identifier` | User-defined names | `start`, `loop`, `data_value` |
| `Number` | Numeric literals | `42`, `0x2A`, `0b101010` |
| `Hash` | Immediate value prefix | `#` |
| `LeftBracket` | Indirect addressing start | `[` |
| `RightBracket` | Indirect addressing end | `]` |
| `Colon` | Label definition | `:` |
| `Comma` | Parameter separator | `,` |
| `Directive` | Assembler directives | `MAKRO`, `KONM` |

### Number Format Support

```rust
// Decimal numbers
let tokens = tokenize("RST 42")?;

// Hexadecimal numbers  
let tokens = tokenize("POB 0xFF")?;

// Binary numbers
let tokens = tokenize("DOD 0b1010")?;

// Negative numbers
let tokens = tokenize("RST -10")?;
```

### Comment Styles

```rust
// Semicolon comments (traditional assembly)
let source = r#"
    POB value    ; This is a comment
    STP          ; Stop the program
"#;

// C-style comments  
let source = r#"
    POB value    // This is also a comment
    STP          // Stop the program
"#;

// Both styles can be mixed
let source = r#"
    ; Program header comment
    start:       // Entry point
        POB #42  ; Load value
        STP      // End program
"#;
```

## 🔧 API Reference

### Main Functions

```rust
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError>;
```

The primary entry point for tokenization. Takes source code and returns a vector of tokens or a lexer error.

### Core Types

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    Number,
    Identifier,
    
    // Keywords and Instructions
    Keyword,
    Directive,
    
    // Symbols
    Hash,           // #
    LeftBracket,    // [  
    RightBracket,   // ]
    Colon,          // :
    Comma,          // ,
    
    // Special
    Newline,
    Invalid,
}

#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    #[error("Invalid character '{char}' at line {line}, column {column}")]
    InvalidCharacter { char: char, line: usize, column: usize },
    
    #[error("Unterminated string literal at line {line}")]
    UnterminatedString { line: usize },
    
    #[error("Invalid number format '{value}' at line {line}, column {column}")]
    InvalidNumberFormat { value: String, line: usize, column: usize },
}
```

### Lexer Class

For more control over tokenization:

```rust
use lexariel::Lexer;

let mut lexer = Lexer::new(source_code);
let tokens = lexer.tokenize()?;

// Access lexer state if needed
println!("Total lines processed: {}", lexer.current_line());
```

## 📖 Examples

### Basic Program Tokenization

```rust
use lexariel::tokenize;

let program = r#"
    ; Simple addition program
    start:
        POB first_num   ; Load first number
        DOD second_num  ; Add second number  
        WYJSCIE         ; Output result
        STP             ; Stop
        
    first_num:  RST 25
    second_num: RST 17
"#;

let tokens = tokenize(program)?;

// Print all tokens with position info
for token in tokens {
    println!("{}:{} - {:?}: '{}'", 
             token.line, token.column, token.kind, token.value);
}
```

### Macro Definition Tokenization

```rust
let macro_source = r#"
    MAKRO multiply_by_two value
        POB value
        DOD value
        WYJSCIE
    KONM
    
    start:
        multiply_by_two data_value
        STP
        
    data_value: RST 21
"#;

let tokens = tokenize(macro_source)?;

// Find macro boundaries
let macro_start = tokens.iter().position(|t| t.value == "MAKRO");
let macro_end = tokens.iter().position(|t| t.value == "KONM");

println!("Macro defined from token {} to {}", 
         macro_start.unwrap(), macro_end.unwrap());
```

### Number Format Recognition

```rust
let numbers_program = r#"
    decimal_val:  RST 42        ; Decimal
    hex_val:      RST 0x2A      ; Hexadecimal  
    binary_val:   RST 0b101010  ; Binary
    negative_val: RST -10       ; Negative
"#;

let tokens = tokenize(numbers_program)?;

// Extract all numbers with their formats
for token in tokens {
    if token.kind == TokenKind::Number {
        println!("Number: '{}' at {}:{}", 
                 token.value, token.line, token.column);
    }
}
```

### Error Handling

```rust
use lexariel::{tokenize, LexerError};

// Source with lexical error
let bad_source = r#"
    start:
        POB @invalid_char   ; @ is not valid
        STP
"#;

match tokenize(bad_source) {
    Ok(tokens) => println!("Tokenized successfully: {} tokens", tokens.len()),
    Err(LexerError::InvalidCharacter { char, line, column }) => {
        println!("Invalid character '{}' at line {}, column {}", char, line, column);
    }
    Err(e) => println!("Other lexer error: {}", e),
}
```

### Addressing Mode Recognition

```rust
let addressing_examples = r#"
    ; Direct addressing
    POB value
    
    ; Immediate addressing  
    POB #42
    
    ; Indirect addressing
    POB [address]
    
    ; Register addressing
    POB R1
"#;

let tokens = tokenize(addressing_examples)?;

// Find addressing mode indicators
for (i, token) in tokens.iter().enumerate() {
    match token.kind {
        TokenKind::Hash => println!("Immediate addressing at token {}", i),
        TokenKind::LeftBracket => println!("Indirect addressing at token {}", i),
        _ => {}
    }
}
```

## 🧪 Testing

### Unit Tests

```bash
cargo test -p lexariel
```

### Specific Test Categories

```bash
# Test basic tokenization
cargo test -p lexariel basic_tokenization

# Test number format recognition  
cargo test -p lexariel number_tests

# Test comment handling
cargo test -p lexariel comment_tests

# Test error recovery
cargo test -p lexariel error_tests
```

### Integration Tests

```bash
cargo test -p lexariel --test integration_tests
```

## 🔍 Performance Characteristics

- **Speed**: ~1M lines per second tokenization
- **Memory**: O(n) where n is source length
- **Error Recovery**: Continues after lexical errors
- **Position Tracking**: Minimal overhead for line/column info

### Benchmarking

```rust
use lexariel::tokenize;
use std::time::Instant;

let large_source = include_str!("large_program.asmod");
let start = Instant::now();
let tokens = tokenize(large_source)?;
let duration = start.elapsed();

println!("Tokenized {} characters into {} tokens in {:?}", 
         large_source.len(), tokens.len(), duration);
```

## 🚫 Error Recovery

Lexariel is designed to continue tokenization even after encountering errors:

```rust
let source_with_errors = r#"
    start:
        POB #42     ; Valid
        @@@         ; Invalid characters
        STP         ; Valid again  
"#;

// Lexer will report error but continue tokenizing
match tokenize(source_with_errors) {
    Ok(tokens) => {
        // Will still get valid tokens before and after error
        println!("Got {} tokens despite errors", tokens.len());
    }
    Err(e) => {
        println!("First error encountered: {}", e);
        // In practice, might want to collect all errors
    }
}
```

## 🔗 Integration with Asmodeus Pipeline

Lexariel is the first stage in the Asmodeus compilation pipeline:

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Source    │───▶│   Lexariel  │───▶│   Parseid   │
│    Code     │    │   (Lexer)   │    │  (Parser)   │
│  (.asmod)   │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘
                            │
                            ▼
                    ┌─────────────┐
                    │   Tokens    │
                    │             │
                    └─────────────┘
```

### Usage in Pipeline

```rust
use lexariel::tokenize;
use parseid::parse;

// Complete pipeline from source to AST
let source = "POB #42\nWYJSCIE\nSTP";
let tokens = tokenize(source)?;      // Lexariel
let ast = parse(tokens)?;            // Parseid
```

## 🎨 Token Visualization

For debugging and development:

```rust
use lexariel::{tokenize, TokenKind};

fn visualize_tokens(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = tokenize(source)?;
    
    println!("┌─────┬────────────┬─────────────┬──────────┐");
    println!("│ Pos │    Type    │    Value    │ Location │");
    println!("├─────┼────────────┼─────────────┼──────────┤");
    
    for (i, token) in tokens.iter().enumerate() {
        println!("│{:4} │{:11} │{:12} │ {:2}:{:<3}   │", 
                 i, 
                 format!("{:?}", token.kind),
                 format!("'{}'", token.value),
                 token.line, 
                 token.column);
    }
    
    println!("└─────┴────────────┴─────────────┴──────────┘");
    Ok(())
}
```

## 🤝 Contributing

### Adding New Token Types

1. Add new variant to `TokenKind` enum
2. Update the lexer logic in `lexer.rs`
3. Add tests for the new token type
4. Update documentation

### Parser Integration

When adding new syntax to Asmodeus:
1. Define tokens in Lexariel
2. Update parser in Parseid to handle new tokens
3. Add assembler support in Hephasm if needed

## 📜 License

This crate is part of the Asmodeus project and is licensed under the MIT License.

## 🔗 Related Components

- **[Parseid](../parseid/)** - Parser that consumes Lexariel tokens
- **[Shared](../shared/)** - Common types and utilities
- **[Main Asmodeus](../)** - Complete language toolchain
