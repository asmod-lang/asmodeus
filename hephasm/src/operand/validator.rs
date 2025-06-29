//! input validation

pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn is_identifier(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let first_char = s.chars().next().unwrap();
        (first_char.is_alphabetic() || first_char == '_') && 
        !s.starts_with("0x") && !s.starts_with("0X") && 
        !s.starts_with("0b") && !s.starts_with("0B") &&
        !s.chars().all(|c| c.is_ascii_digit())
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
