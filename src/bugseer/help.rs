pub fn print_debugger_help() {
    println!("🐛 Bugseer Debugger - Available commands:");
    println!();
    println!("  EXECUTION:");
    println!("    s, step           - Execute one instruction (F7)");
    println!("    n, next           - Execute one instruction (F9) [alias for step]");
    println!("    c, continue       - Continue execution until halt or breakpoint (F10)");
    println!();
    println!("  INSPECTION:");
    println!("    d, display        - Display current machine state");
    println!("    m <start> [end]   - Memory dump (16 words from start, or range)");
    println!();
    println!("  BREAKPOINTS:");
    println!("    b <addr>          - Set breakpoint at address (decimal or hex)");
    println!("    rb <addr>         - Remove breakpoint at address");
    println!("    lb                - List all breakpoints");
    println!();
    println!("  CONTROL:");
    println!("    h, help           - Show this help");
    println!("    q, quit           - Quit debugger");
    println!();
    println!("  ADDRESS FORMATS:");
    println!("    Decimal: 123, 1024");
    println!("    Hexadecimal: 0x7B, 0x400");
    println!();
    println!("🔍 Happy debugging with Bugseer!");
}
