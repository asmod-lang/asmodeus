//! Assembler for Asmodeus assembly language
//! Converts AST into binary machine code for Machine W

use asmodeus_parser::ast::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AssemblerError {
    #[error("Undefined symbol: {symbol} at line {line}")]
    UndefinedSymbol { symbol: String, line: usize },
    #[error("Duplicate symbol definition: {symbol} at line {line}")]
    DuplicateSymbol { symbol: String, line: usize },
    #[error("Invalid opcode: {opcode} at line {line}")]
    InvalidOpcode { opcode: String, line: usize },
    #[error("Invalid number format: {value} at line {line}")]
    InvalidNumber { value: String, line: usize },
    #[error("Address out of bounds: {address} (max 2047) at line {line}")]
    AddressOutOfBounds { address: u16, line: usize },
    #[error("Invalid addressing mode for instruction {instruction}: {mode} at line {line}")]
    InvalidAddressingMode { instruction: String, mode: String, line: usize },
    #[error("Macro not found: {name} at line {line}")]
    MacroNotFound { name: String, line: usize },
    #[error("Macro parameter count mismatch for {name}: expected {expected}, found {found} at line {line}")]
    MacroParameterMismatch { name: String, expected: usize, found: usize, line: usize },
    #[error("Memory overflow: program too large for available memory")]
    MemoryOverflow,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub address: u16,
    pub symbol_type: SymbolType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Label,
    Variable,
}

/// for managing labels and variables
#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, address: u16, symbol_type: SymbolType) -> Result<(), AssemblerError> {
        if self.symbols.contains_key(&name) {
            return Err(AssemblerError::DuplicateSymbol { 
                symbol: name, 
                line: 0 // will be filled by caller
            });
        }
        self.symbols.insert(name, Symbol { address, symbol_type });
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn get_address(&self, name: &str) -> Option<u16> {
        self.symbols.get(name).map(|s| s.address)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ExpandedMacro {
    pub parameters: Vec<String>,
    pub body: Vec<ProgramElement>,
}

pub struct Assembler {
    symbol_table: SymbolTable,
    macros: HashMap<String, ExpandedMacro>,
    memory: Vec<u16>,
    current_address: u16,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            macros: HashMap::new(),
            memory: vec![0; 2048],
            current_address: 0,
        }
    }

    pub fn assemble(&mut self, program: &Program) -> Result<Vec<u16>, AssemblerError> {
        self.symbol_table = SymbolTable::new();
        self.macros.clear();
        self.memory.fill(0);
        self.current_address = 0;

        // first pass: collect macro definitions and expand macro calls
        let expanded_program = self.expand_macros(program)?;

        // second pass
        self.build_symbol_table(&expanded_program)?;

        // third pass: generate machine code
        self.current_address = 0;
        self.generate_code(&expanded_program)?;

        // return only the used portion of memory
        let used_memory = self.current_address as usize;
        Ok(self.memory[0..used_memory].to_vec())
    }

    fn expand_macros(&mut self, program: &Program) -> Result<Vec<ProgramElement>, AssemblerError> {
        let mut expanded = Vec::new();

        for element in &program.elements {
            match element {
                ProgramElement::MacroDefinition(macro_def) => {
                    // store macro definition
                    self.macros.insert(
                        macro_def.name.clone(),
                        ExpandedMacro {
                            parameters: macro_def.parameters.clone(),
                            body: macro_def.body.clone(),
                        },
                    );
                }
                ProgramElement::MacroCall(macro_call) => {
                    let expanded_elements = self.expand_macro_call(macro_call)?;
                    expanded.extend(expanded_elements);
                }
                _ => {
                    // copy other elements as-is
                    expanded.push(element.clone());
                }
            }
        }

        Ok(expanded)
    }

    fn expand_macro_call(&self, macro_call: &MacroCall) -> Result<Vec<ProgramElement>, AssemblerError> {
        let macro_def = self.macros.get(&macro_call.name)
            .ok_or_else(|| AssemblerError::MacroNotFound {
                name: macro_call.name.clone(),
                line: macro_call.line,
            })?;

        if macro_def.parameters.len() != macro_call.arguments.len() {
            return Err(AssemblerError::MacroParameterMismatch {
                name: macro_call.name.clone(),
                expected: macro_def.parameters.len(),
                found: macro_call.arguments.len(),
                line: macro_call.line,
            });
        }

        // parameter substitution map
        let mut substitutions = HashMap::new();
        for (param, arg) in macro_def.parameters.iter().zip(macro_call.arguments.iter()) {
            substitutions.insert(param.clone(), arg.clone());
        }

        // substitute parameters in macro body
        let mut expanded = Vec::new();
        for element in &macro_def.body {
            expanded.push(self.substitute_parameters(element, &substitutions));
        }

        Ok(expanded)
    }

    fn substitute_parameters(&self, element: &ProgramElement, substitutions: &HashMap<String, String>) -> ProgramElement {
        match element {
            ProgramElement::Instruction(inst) => {
                let mut new_inst = inst.clone();
                if let Some(operand) = &mut new_inst.operand {
                    operand.value = self.substitute_in_string(&operand.value, substitutions);
                }
                ProgramElement::Instruction(new_inst)
            }
            ProgramElement::Directive(dir) => {
                let mut new_dir = dir.clone();
                for arg in &mut new_dir.arguments {
                    *arg = self.substitute_in_string(arg, substitutions);
                }
                ProgramElement::Directive(new_dir)
            }
            _ => element.clone(),
        }
    }

    fn substitute_in_string(&self, s: &str, substitutions: &HashMap<String, String>) -> String {
        // entire string matches a parameter = replace it
        if let Some(value) = substitutions.get(s) {
            return value.clone();
        }
        
        // otherwise return the original string
        s.to_string()
    }

    /// second pass
    fn build_symbol_table(&mut self, elements: &[ProgramElement]) -> Result<(), AssemblerError> {
        self.current_address = 0;

        for element in elements {
            match element {
                ProgramElement::LabelDefinition(label) => {
                    self.symbol_table.define(
                        label.name.clone(),
                        self.current_address,
                        SymbolType::Label,
                    ).map_err(|mut e| {
                        if let AssemblerError::DuplicateSymbol { line, .. } = &mut e {
                            *line = label.line;
                        }
                        e
                    })?;
                }
                ProgramElement::Instruction(_) => {
                    self.current_address += 1;
                    if self.current_address > 2048 {
                        return Err(AssemblerError::MemoryOverflow);
                    }
                }
                ProgramElement::Directive(dir) => {
                    match dir.name.to_uppercase().as_str() {
                        "RST" => {
                            self.current_address += 1;
                        }
                        "RPA" => {
                            self.current_address += 1;
                        }
                        _ => {}
                    }
                    if self.current_address > 2048 {
                        return Err(AssemblerError::MemoryOverflow);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// third pass
    fn generate_code(&mut self, elements: &[ProgramElement]) -> Result<(), AssemblerError> {
        self.current_address = 0;

        for element in elements {
            match element {
                ProgramElement::Instruction(inst) => {
                    let machine_code = self.assemble_instruction(inst)?;
                    self.memory[self.current_address as usize] = machine_code;
                    self.current_address += 1;
                }
                ProgramElement::Directive(dir) => {
                    self.assemble_directive(dir)?;
                }
                ProgramElement::LabelDefinition(_) => {
                    // labels dont generate code
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn assemble_instruction(&self, instruction: &Instruction) -> Result<u16, AssemblerError> {
        let opcode = self.get_opcode(&instruction.opcode, instruction.line)?;
        
        let argument = if let Some(operand) = &instruction.operand {
            self.resolve_operand(operand, instruction.line)?
        } else {
            0
        };

        // ensure argument fits in 11 bits
        if argument > 2047 {
            return Err(AssemblerError::AddressOutOfBounds {
                address: argument,
                line: instruction.line,
            });
        }

        // combine opcode (5 bits) and argument (11 bits)
        let machine_code = ((opcode as u16) << 11) | (argument & 0x07FF);
        Ok(machine_code)
    }

    fn get_opcode(&self, instruction: &str, line: usize) -> Result<u8, AssemblerError> {
        match instruction.to_uppercase().as_str() {
            "DOD" => Ok(0b00001),
            "ODE" => Ok(0b00010),
            "ŁAD" | "LAD" => Ok(0b00011),
            "POB" => Ok(0b00100),
            "SOB" => Ok(0b00101),
            "SOM" => Ok(0b00110),
            "STP" => Ok(0b00111),
            "DNS" => Ok(0b01000),
            "PZS" => Ok(0b01001),
            "SDP" => Ok(0b01010),
            "CZM" => Ok(0b01011),
            "MSK" => Ok(0b01100),
            "PWR" => Ok(0b01101),
            "WEJSCIE" => Ok(0b01110),
            "WYJSCIE" => Ok(0b01111),
            _ => Err(AssemblerError::InvalidOpcode {
                opcode: instruction.to_string(),
                line,
            }),
        }
    }

    fn resolve_operand(&self, operand: &Operand, line: usize) -> Result<u16, AssemblerError> {
        // try to resolve as symbol regardless of addressing mode
        if let Some(address) = self.symbol_table.get_address(&operand.value) {
            return Ok(address);
        }

        match &operand.addressing_mode {
            AddressingMode::Immediate => {
                // immediate addressing, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AddressingMode::Direct => {
                // direct addressing, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AddressingMode::Indirect => {
                // indirect addressing, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AddressingMode::Register => {
                // register addressing, extract register number
                self.parse_register(&operand.value, line)
            }
            AddressingMode::Relative => {
                // relative addressing, calculate offset from current position
                let offset = self.parse_signed_number(&operand.value, line)?;
                let target = (self.current_address as i32) + offset;
                if target < 0 || target > 2047 {
                    return Err(AssemblerError::AddressOutOfBounds {
                        address: target as u16,
                        line,
                    });
                }
                Ok(target as u16)
            }
            AddressingMode::Indexed { address, index: _ } => {
                // indexed addressing, try to resolve base address as symbol first
                if let Some(addr) = self.symbol_table.get_address(address) {
                    Ok(addr)
                } else {
                    match self.parse_number(address, line) {
                        Ok(num) => Ok(num),
                        Err(_) if self.is_identifier(address) => {
                            Err(AssemblerError::UndefinedSymbol {
                                symbol: address.clone(),
                                line,
                            })
                        }
                        Err(e) => Err(e),
                    }
                }
            }
            _ => {
                // other addressing modes, parse as number or fail with undefined symbol
                match self.parse_number(&operand.value, line) {
                    Ok(num) => Ok(num),
                    Err(_) if self.is_identifier(&operand.value) => {
                        Err(AssemblerError::UndefinedSymbol {
                            symbol: operand.value.clone(),
                            line,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn is_identifier(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let first_char = s.chars().next().unwrap();
        (first_char.is_alphabetic() || first_char == '_') && 
        !s.starts_with("0x") && !s.starts_with("0X") && 
        !s.starts_with("0b") && !s.starts_with("0B") &&
        !s.chars().all(|c| c.is_ascii_digit())
    }

    fn parse_number(&self, value: &str, line: usize) -> Result<u16, AssemblerError> {
        if value.starts_with("0x") || value.starts_with("0X") {
            // hex
            u16::from_str_radix(&value[2..], 16)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else if value.starts_with("0b") || value.starts_with("0B") {
            // binary
            u16::from_str_radix(&value[2..], 2)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else {
            // decimal
            value.parse::<u16>()
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        }
    }

    /// for relative addressing
    fn parse_signed_number(&self, value: &str, line: usize) -> Result<i32, AssemblerError> {
        let mut is_negative = false;
        let mut num_str = value;
        
        // negative sign
        if value.starts_with('-') {
            is_negative = true;
            num_str = &value[1..];
        }
        
        let result = if num_str.starts_with("0x") || num_str.starts_with("0X") {
            // hex
            i32::from_str_radix(&num_str[2..], 16)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else if num_str.starts_with("0b") || num_str.starts_with("0B") {
            // binary
            i32::from_str_radix(&num_str[2..], 2)
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        } else {
            // decimal
            num_str.parse::<i32>()
                .map_err(|_| AssemblerError::InvalidNumber {
                    value: value.to_string(),
                    line,
                })
        }?;
        
        Ok(if is_negative { -result } else { result })
    }

    fn parse_register(&self, value: &str, line: usize) -> Result<u16, AssemblerError> {
        if !value.to_uppercase().starts_with('R') {
            return Err(AssemblerError::InvalidNumber {
                value: value.to_string(),
                line,
            });
        }

        let reg_num = &value[1..];
        reg_num.parse::<u16>()
            .map_err(|_| AssemblerError::InvalidNumber {
                value: value.to_string(),
                line,
            })
    }

    fn assemble_directive(&mut self, directive: &Directive) -> Result<(), AssemblerError> {
        match directive.name.to_uppercase().as_str() {
            "RST" => {
                let value = if directive.arguments.is_empty() {
                    0
                } else {
                    let signed_value = self.parse_signed_number(&directive.arguments[0], directive.line)?;
                    signed_value as u16
                };
                self.memory[self.current_address as usize] = value;
                self.current_address += 1;
            }
            "RPA" => {
                self.memory[self.current_address as usize] = 0;
                self.current_address += 1;
            }
            _ => {
                // unknown directive - ignoring for now
                // TODO: handle unknown directives
            }
        }
        Ok(())
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

pub fn assemble_source(source: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let program = asmodeus_parser::parse_source(source)?;
    let mut assembler = Assembler::new();
    Ok(assembler.assemble(&program)?)
}

pub fn assemble_program(program: &Program) -> Result<Vec<u16>, AssemblerError> {
    let mut assembler = Assembler::new();
    assembler.assemble(program)
}
