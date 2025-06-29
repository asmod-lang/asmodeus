use lexariel::Token;

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
