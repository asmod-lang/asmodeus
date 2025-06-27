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
    println!("=== MACHINE STATE ===");
    println!("AK: {:04X} ({})    L: {:04X} ({})    AD: {:04X} ({})", 
             state.ak, state.ak, state.l, state.l, state.ad, state.ad);
    println!("KOD: {:02X} ({})      WS: {:04X} ({})    Running: {}", 
             state.kod, state.kod, state.ws, state.ws, state.is_running);
    println!("Interrupts: {}    Mask: {:04X}", 
             state.interrupts_enabled, state.interrupt_mask);
    println!("======================");
}
