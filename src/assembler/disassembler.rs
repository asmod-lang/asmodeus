use std::fs;
use dismael::disassemble;
use crate::error::AsmodeusError;
use crate::cli::Args;
use crate::file_utils::read_binary;

pub fn disassemble_file(input_path: &str, args: &Args) -> Result<(), AsmodeusError> {
    if args.verbose {
        println!("Reading binary file: {}", input_path);
    }
    
    let machine_code = read_binary(input_path)?;
    
    if args.verbose {
        println!("Disassembling {} words of machine code...", machine_code.len());
    }
    
    let assembly = disassemble(&machine_code)?;
    
    let output = assembly.join("\n");
    
    if let Some(output_path) = &args.output_file {
        fs::write(output_path, output).map_err(|e| {
            AsmodeusError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to write output file '{}': {}", output_path, e)
            ))
        })?;
        if args.verbose {
            println!("Disassembly written to: {}", output_path);
        }
    } else {
        println!("{}", output);
    }
    
    Ok(())
}
