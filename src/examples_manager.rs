use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use crate::error::AsmodeusError;
use crate::cli::Args;
use crate::modes::run_mode_run;

#[derive(Debug, Clone)]
struct ExampleInfo {
    name: String,
    title: String,
    path: PathBuf,
    category: String,
}

pub fn handle_examples_command(args: &Args) -> Result<(), AsmodeusError> {
    match args.input_file.as_deref() {
        None | Some("list") => list_examples(),
        Some(sub_command) => {
            let parts: Vec<&str> = sub_command.split_whitespace().collect();
            match parts.as_slice() {
                ["run", example_name] => run_example(example_name, args),
                ["run"] => Err(AsmodeusError::UsageError(
                    "Usage: asmod examples run <example_name>".to_string()
                )),
                [example_name] => {
                    run_example(example_name, args)
                }
                _ => Err(AsmodeusError::UsageError(
                    "Usage: asmod examples [list|run <name>|<name>]".to_string()
                ))
            }
        }
    }
}

pub fn list_examples() -> Result<(), AsmodeusError> {
    println!("📚 Asmodeus Example Programs\n");
    
    let examples = discover_examples()?;
    let categorized = categorize_examples(examples);
    
    if categorized.is_empty() {
        println!("❌ No examples found in the examples/ directory.");
        println!("Make sure you're running from the project root directory.");
        return Ok(());
    }
    
    for (category, items) in categorized {
        println!("{}:", category);
        for example in items {
            println!("  {:<20} - {}", example.name, example.title);
        }
        println!();
    }
    
    println!("💡 Usage:");
    println!("  asmod examples run <name>      # Run example");
    println!("  asmod examples list            # Show this list");
    println!("  asmod examples <name>          # Run example (shortcut)");
    println!();
    println!("🎯 Try: asmod examples run hello");
    
    Ok(())
}

pub fn run_example(name: &str, args: &Args) -> Result<(), AsmodeusError> {
    let examples = discover_examples()?;
    
    for example in examples {
        if example.name == name {
            println!("🚀 Running example: {} - {}\n", example.name, example.title);
            
            let mut example_args = args.clone();
            example_args.input_file = Some(example.path.to_string_lossy().to_string());
            example_args.watch = false;
            
            return run_mode_run(&example_args);
        }
    }
    
    println!("❌ Unknown example: '{}'", name);
    println!("\n📚 Available examples:");
    list_examples()?;
    
    Ok(())
}

fn discover_examples() -> Result<Vec<ExampleInfo>, AsmodeusError> {
    let examples_dir = Path::new("examples");
    if !examples_dir.exists() {
        return Err(AsmodeusError::UsageError(
            "Examples directory not found. Make sure you're running from the project root.".to_string()
        ));
    }
    
    let mut examples = Vec::new();
    discover_examples_recursive(examples_dir, &mut examples)?;
    
    examples.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(examples)
}

fn discover_examples_recursive(dir: &Path, examples: &mut Vec<ExampleInfo>) -> Result<(), AsmodeusError> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                if path.is_dir() {
                    discover_examples_recursive(&path, examples)?;
                } else if path.extension().and_then(|s| s.to_str()) == Some("asmod") {
                    if let Some(example_info) = parse_example_file(&path)? {
                        examples.push(example_info);
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn parse_example_file(path: &Path) -> Result<Option<ExampleInfo>, AsmodeusError> {
    let content = fs::read_to_string(path)
        .map_err(|e| AsmodeusError::IoError(e))?;
    
    let first_line = content.lines().next().unwrap_or("");
    
    // ; example: title
    if let Some(stripped) = first_line.strip_prefix(";") {
        let trimmed = stripped.trim();
        if let Some(example_part) = trimmed.strip_prefix("example:") {
            let title = example_part.trim().to_string();
            
            let name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            let category = determine_category(path);
            
            return Ok(Some(ExampleInfo {
                name,
                title,
                path: path.to_path_buf(),
                category,
            }));
        }
    }
    
    Ok(None)
}

fn determine_category(path: &Path) -> String {
    if let Some(parent) = path.parent() {
        if let Some(dir_name) = parent.file_name().and_then(|s| s.to_str()) {
            return match dir_name {
                "basic" => "🎯 Basic".to_string(),
                "arithmetic" => "🧮 Arithmetic".to_string(),
                "extended_set" => "⚡ Extended Set".to_string(),
                "io" => "💾 I/O Operations".to_string(),
                "arrays" => "📊 Arrays".to_string(),
                "advanced" => "🔧 Advanced".to_string(),
                "errors" => "❌ Error Examples".to_string(),
                _ => format!("📁 {}", dir_name),
            };
        }
    }
    "📄 Other".to_string()
}

fn categorize_examples(examples: Vec<ExampleInfo>) -> HashMap<String, Vec<ExampleInfo>> {
    let mut categorized: HashMap<String, Vec<ExampleInfo>> = HashMap::new();
    
    for example in examples {
        categorized.entry(example.category.clone())
            .or_insert_with(Vec::new)
            .push(example);
    }
    
    for examples in categorized.values_mut() {
        examples.sort_by(|a, b| a.name.cmp(&b.name));
    }
    
    categorized
}
