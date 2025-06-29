use parseid::ast::{Program, ProgramElement};

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
