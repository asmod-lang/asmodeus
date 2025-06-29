use std::fs;
use std::path::Path;
use crate::error::AsmodeusError;
use crate::cli::Mode;

pub fn validate_file_extension(path: &str, mode: Mode) -> Result<(), AsmodeusError> {
    let path = Path::new(path);
    let extension = path.extension().and_then(|ext| ext.to_str());
    
    match (mode.clone(), extension) {
        (Mode::Run | Mode::Assemble | Mode::Debug | Mode::Interactive, Some("asmod")) => Ok(()),
        (Mode::Disassemble, Some("bin")) => Ok(()),
        (Mode::Help, _) => Ok(()), // help mode doesnt need file validation
        (Mode::Examples, _) => Ok(()),
        (Mode::New, _) => Ok(()),
        (Mode::Run | Mode::Assemble | Mode::Debug | Mode::Interactive, Some(ext)) => {
            Err(AsmodeusError::UsageError(
                format!("Expected .asmod, but got .{} file. Please use a valid Asmodeus source file.", ext)
            ))
        }
        (Mode::Disassemble, Some(ext)) => {
            Err(AsmodeusError::UsageError(
                format!("Expected .bin file for disassembly, but got .{} file.", ext)
            ))
        }
        (_, None) => {
            Err(AsmodeusError::UsageError(
                format!("File has no extension. Expected: .asmod for run/assemble/debug, .bin for disassemble")
            ))
        }
    }
}

pub fn read_file(path: &str) -> Result<String, AsmodeusError> {
    fs::read_to_string(path).map_err(|e| {
        AsmodeusError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to read file '{}': {}", path, e)
        ))
    })
}

pub fn write_binary(path: &str, data: &[u16]) -> Result<(), AsmodeusError> {
    let bytes: Vec<u8> = data.iter()
        .flat_map(|&word| word.to_le_bytes())
        .collect();
    fs::write(path, bytes).map_err(|e| {
        AsmodeusError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to write binary file '{}': {}", path, e)
        ))
    })
}

pub fn read_binary(path: &str) -> Result<Vec<u16>, AsmodeusError> {
    let bytes = fs::read(path).map_err(|e| {
        AsmodeusError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to read binary file '{}': {}", path, e)
        ))
    })?;
    
    if bytes.len() % 2 != 0 {
        return Err(AsmodeusError::UsageError(
            format!("Binary file '{}' has odd number of bytes ({})", path, bytes.len())
        ));
    }
    
    let words: Vec<u16> = bytes.chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    
    Ok(words)
}
