use std::fs;
use crate::cli::Args;
use crate::error::AsmodeusError;

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

fn format_assembly_content(content: &str) -> String {

    if content.trim().is_empty() {
        return "\n".to_string();
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut formatted_lines: Vec<String> = Vec::new();
    let indent = "    "; // 4 spaces indendation 
    let mut found_first_code = false;
    let mut i = 0;
    
    while i < lines.len() {
        let trimmed = lines[i].trim();
        
        // skip empty lines 
        if trimmed.is_empty() {
            // empty line compression
            let consecutive_empty = formatted_lines.iter().rev()
                .take_while(|line| line.is_empty())
                .count();
    
            if consecutive_empty < 2 {
                formatted_lines.push(String::new());
            }

            i += 1;
            continue;
        }
        
        if !trimmed.starts_with(';') && !found_first_code {
            found_first_code = true;
        }
        
        // comments 
        if trimmed.starts_with(';') {
            if found_first_code {
                // inline - with code
                formatted_lines.push(format!("{}{}", indent, trimmed));
            } else {
                // on top of the file
                formatted_lines.push(trimmed.to_string());
            }
            i += 1;
            continue;
        }
        
        // labels with instructions (glued together)
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
        
        // just labels (label:)
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
        
        // instructions 
        formatted_lines.push(format!("{}{}", indent, format_instruction(trimmed)));
        i += 1;
    }
    
    while formatted_lines.last() == Some(&String::new()) {
        formatted_lines.pop();
    }

    if !formatted_lines.is_empty() {
        formatted_lines.push(String::new());
    }
    
    formatted_lines.join("\n")
}


fn has_instruction_after_colon(line: &str) -> bool {
    if let Some(colon_pos) = line.find(':') {
        let after_colon = line[colon_pos + 1..].trim();
        
        if !after_colon.is_empty() && !after_colon.starts_with("RST") && !after_colon.starts_with("RPA") {
            if !after_colon.starts_with(';') {
                return true;
            }
        }
    }
    false
}

fn separate_label_and_instruction(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    
    if let Some(colon_pos) = line.find(':') {
        let label = line[..colon_pos].trim();
        let after_colon = line[colon_pos + 1..].trim();
        
        result.push(format!("{}:", label));
        
        if !after_colon.is_empty() {
            result.push(format!("    {}", format_instruction(after_colon)));
        }
    } else {
        result.push(line.to_string());
    }
    
    result
}

fn is_array_start(lines: &[&str], start_idx: usize) -> bool {
    let current_line = lines[start_idx].trim();
    
    if current_line.contains("RST") && current_line.contains(',') {
        return true;
    }
    
    if start_idx + 1 < lines.len() {
        let next_line = lines[start_idx + 1].trim();
        if next_line.starts_with("RST ") {
            return true;
        }
    }
    
    false
}

fn format_array_block(lines: &[&str], current_idx: &mut usize) -> Vec<String> {
    let mut result = Vec::new();
    let first_line = lines[*current_idx].trim();
    
    // first line formatting 
    if let Some(colon_pos) = first_line.find(':') {
        let label = first_line[..colon_pos].trim();
        let data_part = first_line[colon_pos + 1..].trim();
        
        if data_part.contains(',') {
            if let Some(rst_pos) = data_part.find("RST") {
                let after_rst = data_part[rst_pos + 3..].trim();
                let values: Vec<&str> = after_rst.split(',').map(|s| s.trim()).collect();

                result.push(format!("{}: RST {}", label, values[0]));

                let rst_column = label.len() + ": ".len();
                let spaces = " ".repeat(rst_column);
                
                for value in values.iter().skip(1) {
                    result.push(format!("{}RST {}", spaces, value));
                }
            }
        } else {
            result.push(format_label_with_data(first_line));
            
            let rst_column = label.len() + ": ".len();
            let spaces = " ".repeat(rst_column);
            
            *current_idx += 1;
            while *current_idx < lines.len() {
                let next_line = lines[*current_idx].trim();
                if next_line.starts_with("RST ") {
                    let value = next_line[4..].trim(); // usuń "RST "
                    result.push(format!("{}RST {}", spaces, value));
                    *current_idx += 1;
                } else {
                    break;
                }
            }
            *current_idx -= 1;
        }
    }
    
    *current_idx += 1;
    result
}

fn format_instruction(instruction: &str) -> String {
    let (inst_part, comment_part) = split_instruction_and_comment(instruction);
    
    let formatted_inst = if inst_part.contains(' ') {
        let parts: Vec<&str> = inst_part.split_whitespace().collect();
        format!("{} {}", parts[0], parts[1..].join(" "))
    } else {
        inst_part.to_string()
    };
    
    if let Some(comment) = comment_part {
        let fixed_comment = if comment.starts_with(';') && comment.len() > 1 && !comment.chars().nth(1).unwrap().is_whitespace() {
            format!("; {}", &comment[1..])
        } else {
            comment.to_string()
        };
        format!("{:<20} {}", formatted_inst, fixed_comment)
    } else {
        formatted_inst
    }
}

fn format_directive(directive: &str) -> String {
    let (dir_part, comment_part) = split_instruction_and_comment(directive);
    
    let formatted_dir = if dir_part.contains(' ') {
        let parts: Vec<&str> = dir_part.split_whitespace().collect();
        format!("{} {}", parts[0], parts[1..].join(" "))
    } else {
        dir_part.to_string()
    };
    
    if let Some(comment) = comment_part {
        let fixed_comment = if comment.starts_with(';') && comment.len() > 1 && !comment.chars().nth(1).unwrap().is_whitespace() {
            format!("; {}", &comment[1..])
        } else {
            comment.to_string()
        };
        format!("{:<20} {}", formatted_dir, fixed_comment)
    } else {
        formatted_dir
    }
}

fn format_label_with_data(line: &str) -> String {
    if let Some(colon_pos) = line.find(':') {
        let label = line[..colon_pos].trim();
        let data_part = line[colon_pos + 1..].trim();
        
        let (data, comment) = split_instruction_and_comment(data_part);
        
        // one space between instruction and value
        let formatted_data = if data.contains(' ') {
            let parts: Vec<&str> = data.split_whitespace().collect();
            format!("{} {}", parts[0], parts[1..].join(" "))
        } else {
            data.to_string()
        };
        
        let base_line = format!("{}: {}", label, formatted_data);
        
        if let Some(comment) = comment {
            let fixed_comment = if comment.starts_with(';') && comment.len() > 1 && !comment.chars().nth(1).unwrap().is_whitespace() {
                format!("; {}", &comment[1..])
            } else {
                comment.to_string()
            };
            format!("{:<20} {}", base_line, fixed_comment)
        } else {
            base_line
        }
    } else {
        line.to_string()
    }
}

fn split_instruction_and_comment(text: &str) -> (&str, Option<&str>) {
    if let Some(comment_pos) = text.find(';') {
        let inst = text[..comment_pos].trim();
        let comment = text[comment_pos..].trim();
        (inst, Some(comment))
    } else {
        (text.trim(), None)
    }
}

fn save_formatted_content(file_path: &str, content: &str, args: &Args) -> Result<String, std::io::Error> {
    let output_path = if let Some(output) = &args.output_file {
        output.clone()
    } else {
        if file_path.ends_with(".asmod") {
            file_path.replace(".asmod", ".formatted.asmod")
        } else {
            format!("{}.formatted", file_path)
        }
    };
    
    fs::write(&output_path, content)?;
    Ok(output_path)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_comments_no_indent() {
        let input = r#"; example: Test file
; This is a header comment
; Another header comment

start:
    POB #42
    STP"#;

        let expected = r#"; example: Test file
; This is a header comment
; Another header comment

start:
    POB #42
    STP
"#;

        let result = format_assembly_content(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_inline_comments_with_indent() {
        let input = r#"start:
    POB #42    ; this comment should be indented
    STP        ; this too"#;

        let expected = r#"start:
    POB #42              ; this comment should be indented
    STP                  ; this too

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_label_instruction_separation() {
        let input = r#"start:POB first
end:STP"#;

        let expected = r#"start:
    POB first
end:
    STP

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_label_instruction_with_comment_separation() {
        let input = r#"start:POB first;load value
loop:DOD second   ; add value"#;

        let expected = r#"start:
    POB first            ; load value
loop:
    DOD second           ; add value

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_labels_always_left_aligned() {
        let input = r#"    start:
        loop_label:
            end_label:"#;

        let expected = r#"start:
loop_label:
end_label:

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_instruction_formatting() {
        let input = r#"start:
POB    first
DOD      second
    WYJSCIE
        STP"#;

        let expected = r#"start:
    POB first
    DOD second
    WYJSCIE
    STP

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_comment_space_after_semicolon() {
        let input = r#"start:
    POB #42;no space after semicolon
    STP     ;also no space"#;

        let expected = r#"start:
    POB #42              ; no space after semicolon
    STP                  ; also no space

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_simple_data_labels() {
        let input = r#"start:
    POB value
    STP

value:RST 42
message:   RST 100
buffer: RPA"#;

        let expected = r#"start:
    POB value
    STP

value: RST 42
message: RST 100
buffer: RPA

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_array_with_commas() {
        let input = r#"array: RST 10,20,30,40"#;

        let expected = r#"array: RST 10
       RST 20
       RST 30
       RST 40

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_multiline_array() {
        let input = r#"data_array: RST 10
RST 20
RST 30
RST 40
next_label: RST 50"#;

        let expected = r#"data_array: RST 10
            RST 20
            RST 30
            RST 40
next_label: RST 50

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_mixed_arrays_and_labels() {
        let input = r#"first: RST 1
second: RST 2,3,4
third: RST 5
fourth: RST 6
RST 7
RST 8"#;

        let expected = r#"first: RST 1
second: RST 2
        RST 3
        RST 4
third: RST 5
fourth: RST 6
        RST 7
        RST 8

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_directives() {
        let input = r#".data
    .text
.section    .rodata"#;

        let expected = r#"    .data
    .text
    .section .rodata

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
fn test_empty_lines_handling() {
    let input = r#"; header


start:


    POB #42


    STP


"#;

    let expected = r#"; header


start:


    POB #42


    STP

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

    #[test]
    fn test_complex_program() {
        let input = r#"; example: Complex test program
; Tests multiple formatting features

    start:POB first;load first
DOD    second ; add second
    WYJSCIE
STP

loop:    POB counter
SOZ end
    DOD #1
LAD counter
SOB loop

end:STP

first:RST 25
second:    RST 17
array: RST 1,2,3
buffer:RPA
multiline: RST 10
RST 20
RST 30"#;

        let expected = r#"; example: Complex test program
; Tests multiple formatting features

start:
    POB first            ; load first
    DOD second           ; add second
    WYJSCIE
    STP

loop:
    POB counter
    SOZ end
    DOD #1
    LAD counter
    SOB loop

end:
    STP

first: RST 25
second: RST 17
array: RST 1
       RST 2
       RST 3
buffer: RPA
multiline: RST 10
           RST 20
           RST 30

"#;

        let result = format_assembly_content(input);
        assert_eq!(result.trim_end(), expected.trim_end());
    }

    #[test]
    fn test_split_instruction_and_comment() {
        let (inst, comment) = split_instruction_and_comment("POB #42 ; this is comment");
        assert_eq!(inst, "POB #42");
        assert_eq!(comment, Some("; this is comment"));

        let (inst, comment) = split_instruction_and_comment("POB #42;no space");
        assert_eq!(inst, "POB #42");
        assert_eq!(comment, Some(";no space"));

        let (inst, comment) = split_instruction_and_comment("POB #42");
        assert_eq!(inst, "POB #42");
        assert_eq!(comment, None);
    }

    #[test]
    fn test_has_instruction_after_colon() {
        assert_eq!(has_instruction_after_colon("start:POB #42"), true);
        assert_eq!(has_instruction_after_colon("start: POB #42"), true);
        assert_eq!(has_instruction_after_colon("start:"), false);
        assert_eq!(has_instruction_after_colon("value: RST 42"), false);
        assert_eq!(has_instruction_after_colon("value: RPA"), false);
        assert_eq!(has_instruction_after_colon("start:; comment only"), false);
    }

    #[test]
    fn test_is_array_start() {
        let lines = vec!["array: RST 1,2,3"];
        assert_eq!(is_array_start(&lines, 0), true);

        let lines = vec!["array: RST 1", "RST 2"];
        assert_eq!(is_array_start(&lines, 0), true);

        let lines = vec!["value: RST 42"];
        assert_eq!(is_array_start(&lines, 0), false);

        let lines = vec!["value: RST 42", "next: RST 43"];
        assert_eq!(is_array_start(&lines, 0), false);
    }

    #[test]
    fn test_format_instruction() {
        assert_eq!(format_instruction("POB #42"), "POB #42");
        assert_eq!(format_instruction("POB    #42"), "POB #42");
        assert_eq!(format_instruction("POB #42;comment"), "POB #42              ; comment");
        assert_eq!(format_instruction("POB #42   ;comment"), "POB #42              ; comment");
    }

    #[test]
    fn test_separate_label_and_instruction() {
        let result = separate_label_and_instruction("start:POB #42");
        assert_eq!(result, vec!["start:", "    POB #42"]);

        let result = separate_label_and_instruction("start: POB #42");
        assert_eq!(result, vec!["start:", "    POB #42"]);

        let result = separate_label_and_instruction("start:POB #42;comment");
        assert_eq!(result, vec!["start:", "    POB #42              ; comment"]);
    }

    #[test]
    fn test_edge_cases() {
        // empty file
        assert_eq!(format_assembly_content(""), "\n");

        // only comments
        assert_eq!(format_assembly_content("; only comment"), "; only comment\n");

        // just label
        assert_eq!(format_assembly_content("start:"), "start:\n");

        // just instruction
        assert_eq!(format_assembly_content("POB #42"), "    POB #42\n");
    }
}
