pub fn split_instruction_and_comment(text: &str) -> (&str, Option<&str>) {
    if let Some(comment_pos) = text.find(';') {
        let inst = text[..comment_pos].trim();
        let comment = text[comment_pos..].trim();
        (inst, Some(comment))
    } else {
        (text.trim(), None)
    }
}

pub fn fix_comment_spacing(comment: &str) -> String {
    if comment.starts_with(';') && comment.len() > 1 && !comment.chars().nth(1).unwrap().is_whitespace() {
        format!("; {}", &comment[1..])
    } else {
        comment.to_string()
    }
}

pub fn format_line_with_comment(base_content: &str, comment: &str, min_width: usize) -> String {
    let fixed_comment = fix_comment_spacing(comment);
    format!("{:<width$} {}", base_content, fixed_comment, width = min_width)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_fix_comment_spacing() {
        assert_eq!(fix_comment_spacing(";no space"), "; no space");
        assert_eq!(fix_comment_spacing("; already good"), "; already good");
        assert_eq!(fix_comment_spacing(";"), ";");
        assert_eq!(fix_comment_spacing("; "), "; ");
    }

    #[test]
    fn test_format_line_with_comment() {
        let result = format_line_with_comment("POB #42", ";comment", 20);
        assert_eq!(result, "POB #42              ; comment");
        
        let result = format_line_with_comment("POB #42", "; already spaced", 20);
        assert_eq!(result, "POB #42              ; already spaced");
    }
}
