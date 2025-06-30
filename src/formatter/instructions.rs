use crate::formatter::comments::{split_instruction_and_comment, format_line_with_comment};

pub fn format_instruction(instruction: &str) -> String {
    let (inst_part, comment_part) = split_instruction_and_comment(instruction);
    
    // single space between command and argument
    let formatted_inst = normalize_instruction_spacing(inst_part);
    
    // proper alignment if present
    if let Some(comment) = comment_part {
        format_line_with_comment(&formatted_inst, comment, 20)
    } else {
        formatted_inst
    }
}

pub fn format_directive(directive: &str) -> String {
    let (dir_part, comment_part) = split_instruction_and_comment(directive);
    
    // in directive part
    let formatted_dir = normalize_instruction_spacing(dir_part);
    
    // proper alignment if present
    if let Some(comment) = comment_part {
        format_line_with_comment(&formatted_dir, comment, 20)
    } else {
        formatted_dir
    }
}

fn normalize_instruction_spacing(text: &str) -> String {
    if text.contains(' ') {
        let parts: Vec<&str> = text.split_whitespace().collect();
        format!("{} {}", parts[0], parts[1..].join(" "))
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_instruction() {
        assert_eq!(format_instruction("POB #42"), "POB #42");
        assert_eq!(format_instruction("POB    #42"), "POB #42");
        assert_eq!(format_instruction("POB #42;comment"), "POB #42              ; comment");
        assert_eq!(format_instruction("POB #42   ;comment"), "POB #42              ; comment");
        assert_eq!(format_instruction("WYJSCIE"), "WYJSCIE");
    }

    #[test]
    fn test_format_directive() {
        assert_eq!(format_directive(".data"), ".data");
        assert_eq!(format_directive(".text"), ".text");
        assert_eq!(format_directive(".section    .rodata"), ".section .rodata");
        assert_eq!(format_directive(".section .data;comment"), ".section .data       ; comment");
    }

    #[test]
    fn test_normalize_instruction_spacing() {
        assert_eq!(normalize_instruction_spacing("POB #42"), "POB #42");
        assert_eq!(normalize_instruction_spacing("POB    #42"), "POB #42");
        assert_eq!(normalize_instruction_spacing("DOD   first   second"), "DOD first second");
        assert_eq!(normalize_instruction_spacing("STP"), "STP");
    }

    #[test]
    fn test_instruction_with_multiple_args() {
        assert_eq!(format_instruction("DOD   arg1   arg2"), "DOD arg1 arg2");
        assert_eq!(format_instruction("MAKRO   name   param1   param2"), "MAKRO name param1 param2");
    }

    #[test]
    fn test_directive_with_complex_args() {
        assert_eq!(format_directive(".section   .text   \"executable\""), ".section .text \"executable\"");
        assert_eq!(format_directive(".global    _start"), ".global _start");
    }
}
