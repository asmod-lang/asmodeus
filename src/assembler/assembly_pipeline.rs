//! from source to machine code

use lexariel::tokenize;
use parseid::parse;
use hephasm::{assemble_program, assemble_program_extended};

use crate::error::AsmodeusError;
use crate::cli::Args;
use crate::file_utils::read_file;
use crate::debug::{print_tokens_debug, print_ast_debug};

pub fn assemble_file(input_path: &str, args: &Args) -> Result<Vec<u16>, AsmodeusError> {
    if args.verbose {
        println!("Reading source file: {}", input_path);
    }
    
    let source = read_file(input_path)?;
    
    if args.verbose {
        println!("Tokenizing source code...");
    }
    
    let tokens = tokenize(&source).map_err(|e| {
        AsmodeusError::LexerError(e)
    })?;
    
    if args.debug {
        print_tokens_debug(&tokens);
    }
    
    if args.verbose {
        println!("Parsing tokens to AST...");
    }
    
    let ast = parse(tokens).map_err(|e| {
        AsmodeusError::ParserError(e)
    })?;
    
    if args.debug {
        print_ast_debug(&ast);
    }
    
    if args.verbose {
        println!("Assembling AST to machine code...");
    }
    
    let machine_code = if args.extended {
        assemble_program_extended(&ast, true).map_err(|e| {
            AsmodeusError::AssemblerError(e)
        })?
    } else {
        assemble_program(&ast).map_err(|e| {
            AsmodeusError::AssemblerError(e)
        })?
    };
    
    if args.verbose {
        println!("Generated {} words of machine code", machine_code.len());
    }
    
    Ok(machine_code)
}
