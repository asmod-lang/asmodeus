use crate::formatter::{
    labels::{has_instruction_after_colon, separate_label_and_instruction, format_label_with_data},
    arrays::{is_array_start, format_array_block},
    instructions::{format_instruction, format_directive}
};

pub fn format_assembly_content(content: &str) -> String {
    if content.trim().is_empty() {
        return "\n".to_string();
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut formatted_lines: Vec<String> = Vec::new();
    let indent = "    "; // 4 spaces indentation 
    let mut found_first_code = false;
    let mut i = 0;
    
    while i < lines.len() {
        let trimmed = lines[i].trim();
        
        // skip empty lines with compression
        if trimmed.is_empty() {
            handle_empty_line(&mut formatted_lines);
            i += 1;
            continue;
        }
        
        if !trimmed.starts_with(';') && !found_first_code {
            found_first_code = true;
        }
        
        // comments
        if trimmed.starts_with(';') {
            handle_comment(&mut formatted_lines, trimmed, found_first_code, indent);
            i += 1;
            continue;
        }
        
        // labels with instructions (label:INSTRUCTION)
        if trimmed.contains(':') && has_instruction_after_colon(trimmed) {
            let separated = separate_label_and_instruction(trimmed);
            formatted_lines.extend(separated);
            i += 1;
            continue;
        }
        
        // data labels (label: RST/RPA value)
        if trimmed.contains(':') && (trimmed.contains("RST") || trimmed.contains("RPA")) {
            if is_array_start(&lines, i) {
                let processed = format_array_block(&lines, &mut i);
                formatted_lines.extend(processed);
            } else {
                formatted_lines.push(format_label_with_data(trimmed));
                i += 1;
            }
            continue;
        }
        
        // standalone labels (label:)
        if trimmed.ends_with(':') {
            let clean_label = trimmed.replace(' ', "");
            formatted_lines.push(clean_label);
            i += 1;
            continue;
        }
        
        // directives (.directive)
        if trimmed.starts_with('.') {
            formatted_lines.push(format!("{}{}", indent, format_directive(trimmed)));
            i += 1;
            continue;
        }
        
        // regular instructions
        formatted_lines.push(format!("{}{}", indent, format_instruction(trimmed)));
        i += 1;
    }
    
    finalize_output(&mut formatted_lines)
}

fn handle_empty_line(formatted_lines: &mut Vec<String>) {
    let consecutive_empty = formatted_lines.iter().rev()
        .take_while(|line| line.is_empty())
        .count();

    if consecutive_empty < 2 {
        formatted_lines.push(String::new());
    }
}

fn handle_comment(formatted_lines: &mut Vec<String>, trimmed: &str, found_first_code: bool, indent: &str) {
    if found_first_code {
        // after code - indentation
        formatted_lines.push(format!("{}{}", indent, trimmed));
    } else {
        // header comments - no indentation
        formatted_lines.push(trimmed.to_string());
    }
}

fn finalize_output(formatted_lines: &mut Vec<String>) -> String {
    while formatted_lines.last() == Some(&String::new()) {
        formatted_lines.pop();
    }

    if !formatted_lines.is_empty() {
        formatted_lines.push(String::new());
    }
    
    formatted_lines.join("\n")
}
