pub fn print_program_loaded_banner(program_path: &str, word_count: usize) {
    println!();
    
    let left_title = "LOADED PROGRAM";
    let left_file = format!("File: {}", program_path);
    let left_size = format!("Size: {} words ({} bytes)", word_count, word_count * 2);
    let left_content = vec![left_title, &left_file, &left_size];
    
    let right_title = "QUICK HELP";
    let right_help1 = "Type 'h' or 'help' for command list";
    let right_help2 = "Quick: s=step c=continue d=display q=quit";
    let right_content = vec![right_title, right_help1, right_help2];
    
    let left_width = left_content.iter().map(|s| s.len()).max().unwrap() + 4;
    let right_width = right_content.iter().map(|s| s.len()).max().unwrap() + 4;
    let left_box_width = left_width.max(30);
    let right_box_width = right_width.max(30);
    
    let create_border = |width: usize, left: &str, middle: &str, right: &str| -> String {
        format!("{}{}{}", left, middle.repeat(width - 2), right)
    };
    
    let center_text = |text: &str, width: usize, color: &str| -> String {
        let padding = (width - 2 - text.len()) / 2;
        let remaining = width - 2 - text.len() - padding;
        format!("│{}{}{}{}\x1b[0m│",
                " ".repeat(padding),
                color,
                text,
                " ".repeat(remaining))
    };
    
    let center_text_mixed = |text: &str, width: usize, prefix_color: &str, suffix_color: &str| -> String {
        let parts: Vec<&str> = text.splitn(2, ':').collect();
        if parts.len() == 2 {
            let formatted_text = format!("{}{}: {}{}", prefix_color, parts[0], suffix_color, parts[1].trim());
            let text_len = text.len();
            let padding = (width - 2 - text_len) / 2;
            let remaining = width - 2 - text_len - padding;
            format!("│{}{}{}\x1b[0m│",
                    " ".repeat(padding),
                    formatted_text,
                    " ".repeat(remaining))
        } else {
            center_text(text, width, prefix_color)
        }
    };
    
    let center_text_quick = |original_text: &str, width: usize| -> String {
        let parts: Vec<&str> = original_text.splitn(2, ':').collect();
        if parts.len() == 2 {
            let quick_part = format!("\x1b[1m\x1b[38;5;226m{}:", parts[0]);
            let colored_commands = format!(
                "\x1b[1m\x1b[38;5;208ms\x1b[1m\x1b[37m=step \x1b[1m\x1b[38;5;208mc\x1b[1m\x1b[37m=continue \x1b[1m\x1b[38;5;208md\x1b[1m\x1b[37m=display \x1b[1m\x1b[38;5;208mq\x1b[1m\x1b[37m=quit"
            );
            
            let formatted_text = format!("{} {}", quick_part, colored_commands);
            
            let visual_len = original_text.len(); 
            let padding = (width - 2 - visual_len) / 2;
            let remaining = width - 2 - visual_len - padding;
            format!("│{}{}{}\x1b[0m│",
                    " ".repeat(padding),
                    formatted_text,
                    " ".repeat(remaining))
        } else {
            center_text(original_text, width, "\x1b[1m\x1b[38;5;249m")
        }
    };
    
    // top line 
    println!("{}  {}",
             create_border(left_box_width, "┌", "─", "┐"),
             create_border(right_box_width, "┌", "─", "┐"));
    
    // title lines 
    println!("{}  {}",
             center_text(left_content[0], left_box_width, "\x1b[1m\x1b[38;5;69m"),
             center_text(right_content[0], right_box_width, "\x1b[1m\x1b[38;5;227m"));
    
    // separator
    println!("{}  {}",
             create_border(left_box_width, "├", "─", "┤"),
             create_border(right_box_width, "├", "─", "┤"));
    
    // content 
    println!("{}  {}",
             center_text_mixed(left_content[1], left_box_width, "\x1b[1m\x1b[37m", "\x1b[1m\x1b[38;5;117m"),
             center_text(right_content[1], right_box_width, "\x1b[1m\x1b[38;5;249m"));
    
    println!("{}  {}",
             center_text_mixed(left_content[2], left_box_width, "\x1b[1m\x1b[37m", "\x1b[1m\x1b[38;5;245m"),
             center_text_quick(right_content[2], right_box_width)); // używa oryginalnego tekstu
    
    // bottom line 
    println!("{}  {}",
             create_border(left_box_width, "└", "─", "┘"),
             create_border(right_box_width, "└", "─", "┘"));
}
