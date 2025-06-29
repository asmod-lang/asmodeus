pub fn print_program_output(output_buffer: &[u16]) {
    if output_buffer.is_empty() {
        return;
    }
    
    println!();
    
    let title = "OUTPUT";
    let has_multiple_values = output_buffer.len() > 1;
    
    let mut content_lines = Vec::new();
    let mut max_content_len = title.len();
    
    for (i, &value) in output_buffer.iter().enumerate() {
        let line = if has_multiple_values {
            format!("[{}] {} (0x{:04X})", i, value, value)
        } else {
            format!("{} (0x{:04X})", value, value)
        };
        max_content_len = max_content_len.max(line.len());
        content_lines.push(line);
    }
   
    let padding = 4;
    let minimum_content_len = 20;
    let box_width = (max_content_len + padding).max(minimum_content_len);
    
    let create_border = |left: &str, middle: &str, right: &str| -> String {
        format!("{}{}{}", left, middle.repeat(box_width - 2), right)
    };
    
    let center_text = |text: &str, color: &str| -> String {
        let padding = (box_width - 2 - text.len()) / 2;
        let remaining = box_width - 2 - text.len() - padding;
        format!("│{}{}{}{}\x1b[0m│",
                " ".repeat(padding),
                color,
                text,
                " ".repeat(remaining))
    };
    
    let format_line = |original_content: &str, colored_content: &str| -> String {
        let content_len = original_content.len();
        let padding = (box_width - 2 - content_len) / 2;
        let remaining = box_width - 2 - content_len - padding;
        format!("│{}{}{}\x1b[0m│",
                " ".repeat(padding),
                colored_content,
                " ".repeat(remaining))
    };
    
    println!("{}", create_border("┌", "─", "┐"));
    println!("{}", center_text(title, "\x1b[1m\x1b[38;5;249m"));
    println!("{}", create_border("├", "─", "┤"));
    
    for (i, line) in content_lines.iter().enumerate() {
        let colored_line = if has_multiple_values {
            let value = output_buffer[i];
            format!("\x1b[1m\x1b[38;5;240m[{}]\x1b[0m \x1b[1m\x1b[38;5;117m{} (0x{:04X})\x1b[0m", 
                   i, value, value)
        } else {
            let value = output_buffer[0];
            format!("\x1b[1m\x1b[38;5;117m{} (0x{:04X})\x1b[0m", value, value)
        };
        println!("{}", format_line(line, &colored_line));
    }
    println!("{}", create_border("└", "─", "┘"));
}
