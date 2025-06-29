use asmachina::MachineW;

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
