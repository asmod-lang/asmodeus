use crate::formatter::{
    comments::{split_instruction_and_comment, format_line_with_comment},
    instructions::format_instruction
};

pub fn has_instruction_after_colon(line: &str) -> bool {
    if let Some(colon_pos) = line.find(':') {
        let after_colon = line[colon_pos + 1..].trim();
        
        // content after colon thats not RST/RPA (data directives)
        if !after_colon.is_empty() 
            && !after_colon.starts_with("RST") 
            && !after_colon.starts_with("RPA") {
            // not just a comment
            if !after_colon.starts_with(';') {
                return true;
            }
        }
    }
    false
}

pub fn separate_label_and_instruction(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    
    if let Some(colon_pos) = line.find(':') {
        let label = line[..colon_pos].trim();
        let after_colon = line[colon_pos + 1..].trim();
        
        // label (left-aligned)
        result.push(format!("{}:", label));
        
        // instruction with indentation if not empty
        if !after_colon.is_empty() {
            result.push(format!("    {}", format_instruction(after_colon)));
        }
    } else {
        // fallback - just original line
        result.push(line.to_string());
    }
    
    result
}

pub fn format_label_with_data(line: &str) -> String {
    if let Some(colon_pos) = line.find(':') {
        let label = line[..colon_pos].trim();
        let data_part = line[colon_pos + 1..].trim();
        
        let (data, comment) = split_instruction_and_comment(data_part);
        
        // in data part (single space between directive and value)
        let formatted_data = normalize_data_spacing(data);
        
        let base_line = format!("{}: {}", label, formatted_data);
        
        // proper alignment if present
        if let Some(comment) = comment {
            format_line_with_comment(&base_line, comment, 20)
        } else {
            base_line
        }
    } else {
        // Fallback - just original line
        line.to_string()
    }
}

fn normalize_data_spacing(data: &str) -> String {
    if data.contains(' ') {
        let parts: Vec<&str> = data.split_whitespace().collect();
        format!("{} {}", parts[0], parts[1..].join(" "))
    } else {
        data.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_instruction_after_colon() {
        assert_eq!(has_instruction_after_colon("start:POB #42"), true);
        assert_eq!(has_instruction_after_colon("start: POB #42"), true);
        assert_eq!(has_instruction_after_colon("start:"), false);
        assert_eq!(has_instruction_after_colon("value: RST 42"), false);
        assert_eq!(has_instruction_after_colon("value: RPA"), false);
        assert_eq!(has_instruction_after_colon("start:; comment only"), false);
        assert_eq!(has_instruction_after_colon("loop:DOD second"), true);
        assert_eq!(has_instruction_after_colon("end:STP"), true);
    }

    #[test]
    fn test_separate_label_and_instruction() {
        let result = separate_label_and_instruction("start:POB #42");
        assert_eq!(result, vec!["start:", "    POB #42"]);

        let result = separate_label_and_instruction("start: POB #42");
        assert_eq!(result, vec!["start:", "    POB #42"]);

        let result = separate_label_and_instruction("start:POB #42;comment");
        assert_eq!(result, vec!["start:", "    POB #42              ; comment"]);

        let result = separate_label_and_instruction("end:STP");
        assert_eq!(result, vec!["end:", "    STP"]);

        let result = separate_label_and_instruction("label:");
        assert_eq!(result, vec!["label:"]);
    }

    #[test]
    fn test_format_label_with_data() {
        assert_eq!(format_label_with_data("value: RST 42"), "value: RST 42");
        assert_eq!(format_label_with_data("value:RST 42"), "value: RST 42");
        assert_eq!(format_label_with_data("message:   RST 100"), "message: RST 100");
        assert_eq!(format_label_with_data("buffer: RPA"), "buffer: RPA");
        assert_eq!(format_label_with_data("buffer:RPA"), "buffer: RPA");
        
        // With comments
        assert_eq!(format_label_with_data("value: RST 42;comment"), "value: RST 42        ; comment");
        assert_eq!(format_label_with_data("buffer: RPA   ; buffer space"), "buffer: RPA          ; buffer space");
    }

    #[test]
    fn test_normalize_data_spacing() {
        assert_eq!(normalize_data_spacing("RST 42"), "RST 42");
        assert_eq!(normalize_data_spacing("RST    42"), "RST 42");
        assert_eq!(normalize_data_spacing("RPA"), "RPA");
        assert_eq!(normalize_data_spacing("RST   0x2A"), "RST 0x2A");
    }

    #[test]
    fn test_complex_label_scenarios() {
        // multiple spaces
        assert_eq!(format_label_with_data("  value  :  RST   42  "), "value: RST 42");
        
        // hex values
        assert_eq!(format_label_with_data("addr: RST 0x1000"), "addr: RST 0x1000");
        
        // negative values
        assert_eq!(format_label_with_data("offset: RST -10"), "offset: RST -10");
    }
}
