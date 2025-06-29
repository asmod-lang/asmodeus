use crate::error::AssemblerError;

pub struct RegisterParser;

impl RegisterParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_register(&self, value: &str, line: usize) -> Result<u16, AssemblerError> {
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
}

impl Default for RegisterParser {
    fn default() -> Self {
        Self::new()
    }
}
