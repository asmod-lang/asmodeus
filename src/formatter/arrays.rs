use crate::formatter::labels::format_label_with_data;

pub fn is_array_start(lines: &[&str], start_idx: usize) -> bool {
    let current_line = lines[start_idx].trim();
    
    // RST with commas (single-line array)
    if current_line.contains("RST") && current_line.contains(',') {
        return true;
    }
    
    // next line is RST (multi-line array)
    if start_idx + 1 < lines.len() {
        let next_line = lines[start_idx + 1].trim();
        if next_line.starts_with("RST ") {
            return true;
        }
    }
    
    false
}

pub fn format_array_block(lines: &[&str], current_idx: &mut usize) -> Vec<String> {
    let mut result = Vec::new();
    let first_line = lines[*current_idx].trim();
    
    // first line
    if let Some(colon_pos) = first_line.find(':') {
        let label = first_line[..colon_pos].trim();
        let data_part = first_line[colon_pos + 1..].trim();
        
        if data_part.contains(',') {
            // single-line array with commas - split into multiple lines
            format_comma_separated_array(&mut result, label, data_part);
        } else {
            // multi-line array - format first line and continue
            format_multiline_array(&mut result, lines, current_idx, label, first_line);
        }
    }
    
    *current_idx += 1;
    result
}

fn format_comma_separated_array(result: &mut Vec<String>, label: &str, data_part: &str) {
    // split data_part into RST and comment 
    use crate::formatter::comments::split_instruction_and_comment;
    let (rst_data, comment_part) = split_instruction_and_comment(data_part);
    
    if let Some(rst_pos) = rst_data.find("RST") {
        let after_rst = rst_data[rst_pos + 3..].trim();
        let values: Vec<&str> = after_rst.split(',').map(|s| s.trim()).collect();
        
        // first line with label and optional comment
        let first_line = if let Some(comment) = comment_part {
            use crate::formatter::comments::format_line_with_comment;
            format_line_with_comment(&format!("{}: RST {}", label, values[0]), comment, 20)
        } else {
            format!("{}: RST {}", label, values[0])
        };
        result.push(first_line);
        
        // calculate column position for RST alignment
        let rst_column = label.len() + ": ".len();
        let spaces = " ".repeat(rst_column);
        
        // subsequent lines aligned under RST (no comments)
        for value in values.iter().skip(1) {
            result.push(format!("{}RST {}", spaces, value));
        }
    }
}

fn format_multiline_array(
    result: &mut Vec<String>, 
    lines: &[&str], 
    current_idx: &mut usize, 
    label: &str, 
    first_line: &str
) {
    // first line normally
    result.push(format_label_with_data(first_line));
    
    // calculate column position for RST alignment
    let rst_column = label.len() + ": ".len();
    let spaces = " ".repeat(rst_column);
    
    // subsequent RST lines
    *current_idx += 1;
    while *current_idx < lines.len() {
        let next_line = lines[*current_idx].trim();
        if next_line.starts_with("RST ") {
            let value = next_line[4..].trim(); // remove "RST "
            result.push(format!("{}RST {}", spaces, value));
            *current_idx += 1;
        } else {
            break;
        }
    }
    *current_idx -= 1; // because main loop will increment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_array_start() {
        // single-line array with commas
        let lines = vec!["array: RST 1,2,3"];
        assert_eq!(is_array_start(&lines, 0), true);

        // multi-line array
        let lines = vec!["array: RST 1", "RST 2"];
        assert_eq!(is_array_start(&lines, 0), true);

        // not an array
        let lines = vec!["value: RST 42"];
        assert_eq!(is_array_start(&lines, 0), false);

        // not an array
        let lines = vec!["value: RST 42", "next: RST 43"];
        assert_eq!(is_array_start(&lines, 0), false);

        // array at end of file
        let lines = vec!["array: RST 1"];
        assert_eq!(is_array_start(&lines, 0), false);
    }

    #[test]
    fn test_format_comma_separated_array() {
        let mut result = Vec::new();
        format_comma_separated_array(&mut result, "array", "RST 10,20,30,40");
        
        let expected = vec![
            "array: RST 10",
            "       RST 20",
            "       RST 30",
            "       RST 40"
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_multiline_array() {
        let lines = vec!["data_array: RST 10", "RST 20", "RST 30", "RST 40", "next_label: RST 50"];
        let mut current_idx = 0;
        let mut result = Vec::new();
        
        format_multiline_array(&mut result, &lines, &mut current_idx, "data_array", lines[0]);
        
        let expected = vec![
            "data_array: RST 10",
            "            RST 20",
            "            RST 30",
            "            RST 40"
        ];
        assert_eq!(result, expected);
        assert_eq!(current_idx, 3); // should stop before "next_label"
    }

    #[test]
    fn test_format_array_block_with_commas() {
        let lines = vec!["array: RST 10,20,30,40"];
        let mut current_idx = 0;
        
        let result = format_array_block(&lines, &mut current_idx);
        
        let expected = vec![
            "array: RST 10",
            "       RST 20",
            "       RST 30",
            "       RST 40"
        ];
        assert_eq!(result, expected);
        assert_eq!(current_idx, 1);
    }

    #[test]
    fn test_format_array_block_multiline() {
        let lines = vec!["data: RST 10", "RST 20", "RST 30", "next: RST 40"];
        let mut current_idx = 0;
        
        let result = format_array_block(&lines, &mut current_idx);
        
        let expected = vec![
            "data: RST 10",
            "      RST 20",
            "      RST 30"
        ];
        assert_eq!(result, expected);
        assert_eq!(current_idx, 3); // should stop at "next: RST 40"
    }

    #[test]
    fn test_array_with_different_label_lengths() {
        // short label
        let mut result = Vec::new();
        format_comma_separated_array(&mut result, "a", "RST 1,2");
        assert_eq!(result, vec!["a: RST 1", "   RST 2"]);

        // long label
        let mut result = Vec::new();
        format_comma_separated_array(&mut result, "very_long_label", "RST 1,2");
        assert_eq!(result, vec!["very_long_label: RST 1", "                 RST 2"]);
    }
}
