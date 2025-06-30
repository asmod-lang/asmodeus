//! Code formatter for Asmodeus

use std::fs;
use crate::cli::Args;
use crate::error::AsmodeusError;

mod core;
mod labels;
mod arrays;
mod instructions;
mod comments;

#[cfg(test)]
mod tests;

pub use core::format_assembly_content;

pub fn handle_format_command(args: &Args) -> Result<(), AsmodeusError> {
    let file_path = match &args.input_file {
        Some(path) => path,
        None => {
            eprintln!("Error: No file specified for formatting");
            return Err(AsmodeusError::UsageError("No input file specified".to_string()));
        }
    };

    if args.verbose {
        println!("🎨 Formatting: {}", file_path);
    }

    match format_file(file_path, args) {
        Ok(formatted_content) => {
            match save_formatted_content(file_path, &formatted_content, args) {
                Ok(output_path) => {
                    if args.verbose {
                        println!("✅ File formatted successfully");
                        println!("  Output: {}", output_path);
                    } else {
                        println!("✅ Formatted: {}", output_path);
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error saving formatted file: {}", e);
                    Err(AsmodeusError::IoError(e))
                }
            }
        }
        Err(e) => {
            eprintln!("Error formatting file: {}", e);
            Err(AsmodeusError::UsageError(e))
        }
    }
}

fn format_file(file_path: &str, _args: &Args) -> Result<String, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Could not read file: {}", e))?;

    let formatted = format_assembly_content(&content);
    Ok(formatted)
}

fn save_formatted_content(file_path: &str, content: &str, args: &Args) -> Result<String, std::io::Error> {
    let output_path = if let Some(output) = &args.output_file {
        output.clone()
    } else {
        if file_path.ends_with(".asmod") {
            file_path.replace(".asmod", "-formatted.asmod")
        } else {
            format!("{}.formatted", file_path)
        }
    };
    
    fs::write(&output_path, content)?;
    Ok(output_path)
}
