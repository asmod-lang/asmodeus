//! Parser for Asmodeus assembly language
//! Converts tokens into Abstract Syntax Tree (AST)

pub mod ast;

use asmodeus_lexer::{Token, TokenKind, LexerError};
use ast::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Unexpected token at line {line}, column {column}: expected {expected}, found {found}")]
    UnexpectedToken {
        line: usize,
        column: usize,
        expected: String,
        found: String,
    },
    #[error("Unexpected end of file, expected {expected}")]
    UnexpectedEof { expected: String },
    #[error("Invalid addressing mode at line {line}, column {column}: {mode}")]
    InvalidAddressingMode {
        line: usize,
        column: usize,
        mode: String,
    },
    #[error("Missing operand for instruction {instruction} at line {line}, column {column}")]
    MissingOperand {
        instruction: String,
        line: usize,
        column: usize,
    },
    #[error("Invalid macro definition at line {line}, column {column}: {message}")]
    InvalidMacroDefinition {
        line: usize,
        column: usize,
        message: String,
    },
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// creates a new parser with the given tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    /// returns the current token without advancing
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// returns the token at the given offset without advancing
    fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    /// advances to the next token and returns it
    fn advance(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    /// checks if the current token matches the expected kind
    fn check(&self, kind: TokenKind) -> bool {
        if let Some(token) = self.peek() {
            token.kind == kind
        } else {
            false
        }
    }

    /// consumes a token of the expected kind or returns an error
    fn consume(&mut self, kind: TokenKind, expected: &str) -> Result<Token, ParserError> {
        if let Some(token) = self.peek() {
            if token.kind == kind {
                Ok(self.advance().unwrap().clone())
            } else {
                Err(ParserError::UnexpectedToken {
                    line: token.line,
                    column: token.column,
                    expected: expected.to_string(),
                    found: format!("{}", token.kind),
                })
            }
        } else {
            Err(ParserError::UnexpectedEof {
                expected: expected.to_string(),
            })
        }
    }

    /// parses the entire program
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::new();

        while let Some(token) = self.peek() {
            if token.kind == TokenKind::Eof {
                break;
            }

            let element = self.parse_program_element()?;
            program.add_element(element);
        }

        Ok(program)
    }

    /// parses a top-level program element
    fn parse_program_element(&mut self) -> Result<ProgramElement, ParserError> {
        match self.peek() {
            Some(token) => match &token.kind {
                TokenKind::LabelDef => {
                    let label = self.parse_label_definition()?;
                    Ok(ProgramElement::LabelDefinition(label))
                }
                TokenKind::Directive => {
                    if token.value.to_uppercase() == "MAKRO" {
                        let macro_def = self.parse_macro_definition()?;
                        Ok(ProgramElement::MacroDefinition(macro_def))
                    } else {
                        let directive = self.parse_directive()?;
                        Ok(ProgramElement::Directive(directive))
                    }
                }
                TokenKind::Keyword => {
                    let instruction = self.parse_instruction()?;
                    Ok(ProgramElement::Instruction(instruction))
                }
                TokenKind::Identifier => {
                    let macro_call = self.parse_macro_call()?;
                    Ok(ProgramElement::MacroCall(macro_call))
                }
                _ => Err(ParserError::UnexpectedToken {
                    line: token.line,
                    column: token.column,
                    expected: "instruction, directive, or label".to_string(),
                    found: format!("{}", token.kind),
                }),
            },
            None => Err(ParserError::UnexpectedEof {
                expected: "program element".to_string(),
            }),
        }
    }

    fn parse_label_definition(&mut self) -> Result<LabelDefinition, ParserError> {
        let token = self.consume(TokenKind::LabelDef, "label definition")?;
        Ok(LabelDefinition {
            name: token.value,
            line: token.line,
            column: token.column,
        })
    }

    fn parse_instruction(&mut self) -> Result<Instruction, ParserError> {
        let token = self.consume(TokenKind::Keyword, "instruction")?;
        let opcode = token.value.clone();
        let line = token.line;
        let column = token.column;

        let operand = if self.has_operand(&opcode) {
            Some(self.parse_operand()?)
        } else {
            None
        };

        Ok(Instruction {
            opcode,
            operand,
            line,
            column,
        })
    }

    fn has_operand(&self, opcode: &str) -> bool {
        match opcode.to_uppercase().as_str() {
            "STP" | "DNS" | "PZS" | "SDP" | "CZM" | "PWR" | "WEJSCIE" | "WYJSCIE" => false,
            _ => true,
        }
    }

    /// parses an operand with addressing mode
    fn parse_operand(&mut self) -> Result<Operand, ParserError> {
        // do we have a token for operand?
        if self.peek().is_none() || self.peek().unwrap().kind == TokenKind::Eof {
            return Err(ParserError::UnexpectedEof {
                expected: "operand".to_string(),
            });
        }

        // check if the next token is something that would start a new statement
        let token = self.peek().unwrap();
        match token.kind {
            TokenKind::LabelDef | TokenKind::Keyword | TokenKind::Directive => {
                return Err(ParserError::UnexpectedEof {
                    expected: "operand".to_string(),
                });
            }
            _ => {}
        }

        let token = self.peek().unwrap();

        match &token.kind {
            TokenKind::Punctuation if token.value == "#" => {
                // immediate addressing: #value
                self.advance(); // consume #
                let value_token = self.consume_value("immediate value")?;
                Ok(Operand {
                    addressing_mode: AddressingMode::Immediate,
                    value: value_token.value,
                })
            }
            TokenKind::Punctuation if token.value == "[" => {
                // indirect or multiple indirect addressing
                self.parse_indirect_operand()
            }
            TokenKind::Identifier if token.value.to_uppercase().starts_with('R') => {
                // register addressing
                self.parse_register_operand()
            }
            TokenKind::Punctuation if token.value == "+" || token.value == "-" => {
                // relative addressing
                let sign_token = self.advance().unwrap().clone();
                let offset_token = self.consume_value("offset value")?;
                Ok(Operand {
                    addressing_mode: AddressingMode::Relative,
                    value: format!("{}{}", sign_token.value, offset_token.value),
                })
            }
            TokenKind::Number => {
                // could be negative number (relative) or positive number (direct)
                let number_token = self.advance().unwrap().clone();
                if number_token.value.starts_with('-') {
                    Ok(Operand {
                        addressing_mode: AddressingMode::Relative,
                        value: number_token.value,
                    })
                } else {
                    // indexed addressing: number[index]
                    if self.check(TokenKind::Punctuation) && self.peek().unwrap().value == "[" {
                        self.advance(); // consume [
                        let index_token = self.consume_value("index")?;
                        let index_value = index_token.value.clone();
                        self.consume(TokenKind::Punctuation, "]")?;
                        
                        Ok(Operand {
                            addressing_mode: AddressingMode::Indexed {
                                address: number_token.value.clone(),
                                index: index_value.clone(),
                            },
                            value: format!("{}[{}]", number_token.value, index_value),
                        })
                    } else {
                        // direct addressing
                        Ok(Operand {
                            addressing_mode: AddressingMode::Direct,
                            value: number_token.value,
                        })
                    }
                }
            }
            _ => {
                // direct addressing or indexed addressing (identifiers)
                self.parse_direct_or_indexed_operand()
            }
        }
    } 

    fn parse_indirect_operand(&mut self) -> Result<Operand, ParserError> {
        self.consume(TokenKind::Punctuation, "[")?;

        // multiple indirect [[...]]
        if self.check(TokenKind::Punctuation) && self.peek().unwrap().value == "[" {
            self.advance(); // consume second [
            let value_token = self.consume_value("address")?;
            let value = value_token.value.clone();
            self.consume(TokenKind::Punctuation, "]")?; // consume first ]
            self.consume(TokenKind::Punctuation, "]")?; // consume second ]
            
            Ok(Operand {
                addressing_mode: AddressingMode::MultipleIndirect,
                value,
            })
        } else {
            // register indirect [R0] or regular indirect [address]
            let value_token = self.peek().ok_or(ParserError::UnexpectedEof {
                expected: "address or register".to_string(),
            })?;

            if value_token.kind == TokenKind::Identifier && value_token.value.to_uppercase().starts_with('R') {
                // indirect [R0]
                let reg_token = self.advance().unwrap().clone();
                self.consume(TokenKind::Punctuation, "]")?;
                
                Ok(Operand {
                    addressing_mode: AddressingMode::RegisterIndirect,
                    value: reg_token.value,
                })
            } else {
                // indirect [address]
                let addr_token = self.consume_value("address")?;
                let value = addr_token.value.clone();
                self.consume(TokenKind::Punctuation, "]")?;
                
                Ok(Operand {
                    addressing_mode: AddressingMode::Indirect,
                    value,
                })
            }
        }
    }

    fn parse_register_operand(&mut self) -> Result<Operand, ParserError> {
        let reg_token = self.consume(TokenKind::Identifier, "register")?;
        let reg_name = reg_token.value.clone();
        
        // base register addressing R0[offset]
        if self.check(TokenKind::Punctuation) && self.peek().unwrap().value == "[" {
            self.advance(); // consume [
            let offset_token = self.consume_value("offset")?;
            let offset_value = offset_token.value.clone();
            self.consume(TokenKind::Punctuation, "]")?;
            
            Ok(Operand {
                addressing_mode: AddressingMode::BaseRegister {
                    base: reg_name.clone(),
                    offset: offset_value.clone(),
                },
                value: format!("{}[{}]", reg_name, offset_value),
            })
        } else {
            // simple register addressing
            Ok(Operand {
                addressing_mode: AddressingMode::Register,
                value: reg_name,
            })
        }
    }

    fn parse_direct_or_indexed_operand(&mut self) -> Result<Operand, ParserError> {
        let addr_token = self.consume_value("address")?;
        let addr_value = addr_token.value.clone();
        
        // indexed addressing address[index]
        if self.check(TokenKind::Punctuation) && self.peek().unwrap().value == "[" {
            self.advance(); // consume [
            let index_token = self.consume_value("index")?;
            let index_value = index_token.value.clone();
            self.consume(TokenKind::Punctuation, "]")?;
            
            Ok(Operand {
                addressing_mode: AddressingMode::Indexed {
                    address: addr_value.clone(),
                    index: index_value.clone(),
                },
                value: format!("{}[{}]", addr_value, index_value),
            })
        } else {
            // direct addressing
            Ok(Operand {
                addressing_mode: AddressingMode::Direct,
                value: addr_value,
            })
        }
    }

    /// consumes a value token (number or identifier)
    fn consume_value(&mut self, expected: &str) -> Result<Token, ParserError> {
        let token = self.peek().ok_or(ParserError::UnexpectedEof {
            expected: expected.to_string(),
        })?;

        match token.kind {
            TokenKind::Number | TokenKind::Identifier => Ok(self.advance().unwrap().clone()),
            _ => Err(ParserError::UnexpectedToken {
                line: token.line,
                column: token.column,
                expected: expected.to_string(),
                found: format!("{}", token.kind),
            }),
        }
    }

    fn parse_directive(&mut self) -> Result<Directive, ParserError> {
        let token = self.consume(TokenKind::Directive, "directive")?;
        let name = token.value;
        let line = token.line;
        let column = token.column;

        let mut arguments = Vec::new();

        while let Some(arg_token) = self.peek() {
            match arg_token.kind {
                TokenKind::Number | TokenKind::Identifier => {
                    arguments.push(self.advance().unwrap().value.clone());
                }
                TokenKind::Eof | TokenKind::LabelDef | TokenKind::Keyword | TokenKind::Directive => {
                    break;
                }
                _ => {
                    self.advance(); // skip other tokens (...punctuation)
                }
            }
        }

        Ok(Directive {
            name,
            arguments,
            line,
            column,
        })
    }

    fn parse_macro_definition(&mut self) -> Result<MacroDefinition, ParserError> {
        let makro_token = self.consume(TokenKind::Directive, "MAKRO")?;
        let line = makro_token.line;
        let column = makro_token.column;

        let name_token = self.consume(TokenKind::Identifier, "macro name")?;
        let name = name_token.value;

        let mut parameters = Vec::new();
        let mut body = Vec::new();

        // parameters (optional)
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Identifier => {
                    parameters.push(self.advance().unwrap().value.clone());
                }
                TokenKind::Directive if token.value.to_uppercase() == "KONM" => {
                    break;
                }
                TokenKind::Keyword | TokenKind::LabelDef => {
                    break;
                }
                _ => {
                    self.advance(); // skip other tokens
                }
            }
        }

        // until KONM
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::Directive && token.value.to_uppercase() == "KONM" {
                self.advance(); // consume KONM
                break;
            }
            if token.kind == TokenKind::Eof {
                return Err(ParserError::InvalidMacroDefinition {
                    line,
                    column,
                    message: "Missing KONM to end macro definition".to_string(),
                });
            }
            body.push(self.parse_program_element()?);
        }

        Ok(MacroDefinition {
            name,
            parameters,
            body,
            line,
            column,
        })
    }

    fn parse_macro_call(&mut self) -> Result<MacroCall, ParserError> {
        let token = self.consume(TokenKind::Identifier, "macro name")?;
        let name = token.value;
        let line = token.line;
        let column = token.column;

        let mut arguments = Vec::new();

        // arguments
        while let Some(arg_token) = self.peek() {
            match arg_token.kind {
                TokenKind::Number | TokenKind::Identifier => {
                    arguments.push(self.advance().unwrap().value.clone());
                }
                TokenKind::Eof | TokenKind::LabelDef | TokenKind::Keyword | TokenKind::Directive => {
                    break;
                }
                _ => {
                    self.advance(); // skip other tokens
                }
            }
        }

        Ok(MacroCall {
            name,
            arguments,
            line,
            column,
        })
    }
}

/// convenience function to parse tokens into AST
pub fn parse(tokens: Vec<Token>) -> Result<Program, ParserError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// convenience function to parse source code directly
pub fn parse_source(source: &str) -> Result<Program, ParserError> {
    let tokens = asmodeus_lexer::tokenize(source)?;
    parse(tokens)
}
