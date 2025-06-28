//! debug and diagnostic utilities

use lexariel::Token;
use parseid::ast::{Program, ProgramElement};
use asmachina::MachineW;

pub fn print_tokens_debug(tokens: &[Token]) {
    println!("=== TOKENS ===");
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {} {{", i + 1);
        println!("    kind: {:?},", token.kind);
        println!("    value: \"{}\",", token.value);
        println!("    line: {},", token.line);
        println!("    column: {}", token.column);
        println!("}},");
    }
    println!("======\n");
}

pub fn print_ast_debug(ast: &Program) {
    println!("=== AST ===");
    println!("Program {{");
    println!("    elements: [");
    for (i, element) in ast.elements.iter().enumerate() {
        print!("        ");
        match element {
            ProgramElement::LabelDefinition(label) => {
                println!("LabelDefinition(");
                println!("            LabelDefinition {{");
                println!("                name: \"{}\",", label.name);
                println!("                line: {},", label.line);
                println!("                column: {}", label.column);
                println!("            }}");
                print!("        )");
            },
            ProgramElement::Instruction(instr) => {
                println!("Instruction(");
                println!("            Instruction {{");
                println!("                opcode: \"{}\",", instr.opcode);
                match &instr.operand {
                    Some(operand) => {
                        println!("                operand: Some(Operand {{");
                        println!("                    addressing_mode: {:?},", operand.addressing_mode);
                        println!("                    value: \"{}\"", operand.value);
                        println!("                }}),");
                    },
                    None => println!("                operand: None,"),
                }
                println!("                line: {},", instr.line);
                println!("                column: {}", instr.column);
                println!("            }}");
                print!("        )");
            },
            ProgramElement::Directive(directive) => {
                println!("Directive(");
                println!("            Directive {{");
                println!("                name: \"{}\",", directive.name);
                println!("                arguments: {:?},", directive.arguments);
                println!("                line: {},", directive.line);
                println!("                column: {}", directive.column);
                println!("            }}");
                print!("        )");
            }
            ProgramElement::MacroDefinition(_) => {
                println!("MacroDefinition(...)");
                print!("        ");
            }
            ProgramElement::MacroCall(_) => {
                println!("MacroCall(...)");
                print!("        ");
            }
        }
        if i < ast.elements.len() - 1 {
            println!(",");
        } else {
            println!();
        }
    }
    println!("    ]");
    println!("}}");
    println!("======\n");
}

pub fn print_machine_state(machine: &MachineW) {
    let state = machine.get_current_state();
    
    println!();
    
    let title = "MACHINE STATE";
    let ak_line = format!("AK: {:04X} ({})    L: {:04X} ({})    AD: {:04X} ({})", 
                         state.ak, state.ak, state.l, state.l, state.ad, state.ad);
    let kod_ws_line = format!("KOD: {:02X} ({})      WS: {:04X} ({})", 
                             state.kod, state.kod, state.ws, state.ws);
    let interrupts_line = format!("Interrupts: {}    Mask: {:04X}", 
                                 state.interrupts_enabled, state.interrupt_mask);
    let running_line = format!("Running: {}", state.is_running);
   
    let padding = 4;
    let content_width = [
        title.len(),
        ak_line.len(),
        kod_ws_line.len(),
        interrupts_line.len(),
        running_line.len()
    ].iter().max().unwrap() + padding;
    
    let box_width = content_width.max(50);
    
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
        let padding = 2;
        let total_used = padding + original_content.len();
        let remaining = if total_used < box_width - 2 {
            box_width - 2 - total_used
        } else {
            0
        };
        format!("│{}{}{}\x1b[0m│",
                " ".repeat(padding),
                colored_content,
                " ".repeat(remaining))
    };
    
    println!("{}", create_border("┌", "─", "┐"));
    println!("{}", center_text(title, "\x1b[1m\x1b[38;5;214m"));
    println!("{}", create_border("├", "─", "┤"));
    
    let ak_colored = format!("\x1b[1m\x1b[38;5;33mAK:\x1b[0m \x1b[1m\x1b[37m{:04X}\x1b[0m \x1b[1m\x1b[38;5;117m({})\x1b[0m    \x1b[1m\x1b[38;5;33mL:\x1b[0m \x1b[1m\x1b[37m{:04X}\x1b[0m \x1b[1m\x1b[38;5;117m({})\x1b[0m    \x1b[1m\x1b[38;5;33mAD:\x1b[0m \x1b[1m\x1b[37m{:04X}\x1b[0m \x1b[1m\x1b[38;5;117m({})\x1b[0m",
                            state.ak, state.ak, state.l, state.l, state.ad, state.ad);
    println!("{}", format_line(&ak_line, &ak_colored));
    
    let kod_ws_colored = format!("\x1b[1m\x1b[38;5;33mKOD:\x1b[0m \x1b[1m\x1b[37m{:02X}\x1b[0m \x1b[1m\x1b[38;5;117m({})\x1b[0m      \x1b[1m\x1b[38;5;33mWS:\x1b[0m \x1b[1m\x1b[37m{:04X}\x1b[0m \x1b[1m\x1b[38;5;117m({})\x1b[0m",
                                 state.kod, state.kod, state.ws, state.ws);
    println!("{}", format_line(&kod_ws_line, &kod_ws_colored));
    
    println!("│{}│", " ".repeat(box_width - 2));
    
    let interrupts_color = if state.interrupts_enabled { "\x1b[1m\x1b[38;5;120m" } else { "\x1b[1m\x1b[38;5;210m" };
    let interrupts_colored = format!("\x1b[1m\x1b[38;5;208mInterrupts:\x1b[0m {}{}{}    \x1b[1m\x1b[38;5;208mMask:\x1b[0m \x1b[1m\x1b[37m{:04X}\x1b[0m",
                                    interrupts_color, state.interrupts_enabled, "\x1b[0m", state.interrupt_mask);
    println!("{}", format_line(&interrupts_line, &interrupts_colored));
    
    println!("│{}│", " ".repeat(box_width - 2));
    
    let running_color = if state.is_running { "\x1b[1m\x1b[38;5;28m" } else { "\x1b[1m\x1b[38;5;168m" };
    let running_colored = format!("\x1b[1m\x1b[38;5;218mRunning:\x1b[0m {}{}{}", 
                                 running_color, state.is_running, "\x1b[0m");
    println!("{}", format_line(&running_line, &running_colored));
    
    println!("{}", create_border("└", "─", "┘"));
}

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
