use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::error::AsmodeusError;
use crate::cli::Args;

#[derive(Debug, Clone)]
struct TemplateInfo {
    name: String,
    title: String,
    description: String,
    category: String,
    content: String,
    requires_extended: bool,
    interactive_mode: bool,
}

pub fn handle_new_command(args: &Args) -> Result<(), AsmodeusError> {
    // "project_name --template template_name" or "project_name"
    match args.input_file.as_deref() {
        None => show_template_help(),
        Some(arguments) => {
            let parts: Vec<&str> = arguments.split_whitespace().collect();
            match parts.as_slice() {
                ["--list"] => list_templates(),
                [project_name] => create_new_project(project_name, "hello", args),
                [project_name, "--template", template_name] => create_new_project(project_name, template_name, args),
                _ => Err(AsmodeusError::UsageError(
                    "Usage: asmod new <project_name> [--template <template_name>]".to_string()
                ))
            }
        }
    }
}

pub fn create_new_project(name: &str, template_name: &str, args: &Args) -> Result<(), AsmodeusError> {
    let filename = format!("{}.asmod", name);
    
    if Path::new(&filename).exists() {
        return Err(AsmodeusError::UsageError(
            format!("File '{}' already exists. Choose a different name.", filename)
        ));
    }
    
    let template = find_template(template_name)?;
    let content = generate_content_from_template(&template, name)?;
    
    fs::write(&filename, &content).map_err(|e| AsmodeusError::IoError(e))?;
    
    println!("✅ Created new Asmodeus project: {}", filename);
    println!("📝 Template: {} - {}", template.name, template.title);
    println!();
    println!("🚀 Next steps:");
    
    if template.requires_extended {
        println!("   asmod run --extended {}     # Run with extended instructions", filename);
        println!("   asmod --watch --extended {} # Watch with extended mode", filename);
    } else if template.interactive_mode {
        println!("   asmod interactive {}        # Run in interactive mode", filename);
        println!("   asmod --watch {}           # Watch and auto-reload", filename);
    } else {
        println!("   asmod run {}               # Run your program", filename);
        println!("   asmod --watch {}           # Watch and auto-reload", filename);
    }
    println!("   asmod debug {}             # Debug your program", filename);
    
    if args.verbose {
        println!("\n📄 Generated content preview:");
        println!("{}", "-".repeat(50));
        let preview: String = content.lines().take(15).collect::<Vec<_>>().join("\n");
        println!("{}", preview);
        if content.lines().count() > 15 {
            println!("... ({} more lines)", content.lines().count() - 15);
        }
        println!("{}", "-".repeat(50));
    }
    
    Ok(())
}

pub fn list_templates() -> Result<(), AsmodeusError> {
    println!("📝 Available Asmodeus Templates\n");
    
    let templates = discover_templates()?;
    let categorized = categorize_templates(templates);
    
    if categorized.is_empty() {
        println!("❌ No templates found in the templates/ directory.");
        println!("Make sure you're running from the project root directory.");
        return Ok(());
    }
    
    for (category, items) in categorized {
        println!("{}:", category);
        for template in items {
            let flags = if template.requires_extended {
                " [extended]"
            } else if template.interactive_mode {
                " [interactive]"
            } else {
                ""
            };
            println!("  {:<15} - {}{}", template.name, template.description, flags);
        }
        println!();
    }
    
    println!("💡 Usage:");
    println!("  asmod new <name>                        # Create with default template");
    println!("  asmod new <name> --template <template>  # Create with specific template");
    println!("  asmod new --list                        # Show this list");
    println!();
    println!("🎯 Try: asmod new my_project --template calc");
    
    Ok(())
}

fn show_template_help() -> Result<(), AsmodeusError> {
    println!("📝 Asmodeus Project Generator\n");
    println!("Usage: asmod new <project_name> [--template <template_name>]\n");
    println!("Options:");
    println!("  --template <name>  Use specific template (default: hello)");
    println!("  --list             List available templates");
    println!();
    println!("Examples:");
    println!("  asmod new hello                    # Create hello.asmod with default template");
    println!("  asmod new calc --template calc     # Create calc.asmod with calculator template");
    println!("  asmod new --list                   # Show available templates");
    
    Ok(())
}

fn find_template(template_name: &str) -> Result<TemplateInfo, AsmodeusError> {
    let templates = discover_templates()?;
    
    for template in templates {
        if template.name == template_name {
            return Ok(template);
        }
    }
    
    Err(AsmodeusError::UsageError(
        format!("Unknown template: '{}'. Use 'asmod new --list' to see available templates.", template_name)
    ))
}

fn discover_templates() -> Result<Vec<TemplateInfo>, AsmodeusError> {
    let templates_dir = Path::new("templates");
    if !templates_dir.exists() {
        return Err(AsmodeusError::UsageError(
            "Templates directory not found. Make sure you're running from the project root.".to_string()
        ));
    }
    
    let mut templates = Vec::new();
    discover_templates_recursive(templates_dir, &mut templates)?;
    
    templates.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(templates)
}

fn discover_templates_recursive(dir: &Path, templates: &mut Vec<TemplateInfo>) -> Result<(), AsmodeusError> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                if path.is_dir() {
                    discover_templates_recursive(&path, templates)?;
                } else if path.extension().and_then(|s| s.to_str()) == Some("asmod") {
                    if let Some(template_info) = parse_template_file(&path)? {
                        templates.push(template_info);
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn parse_template_file(path: &Path) -> Result<Option<TemplateInfo>, AsmodeusError> {
    let content = fs::read_to_string(path)
        .map_err(|e| AsmodeusError::IoError(e))?;
    
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Ok(None);
    }
    
    let mut title = String::new();
    let mut description = String::new();
    let mut requires_extended = false;
    let mut interactive_mode = false;
    let mut content_start = 0;
    
    for (i, line) in lines.iter().enumerate() {
        let line = line.trim();
        
        if let Some(stripped) = line.strip_prefix(";") {
            let trimmed = stripped.trim();
            
            if let Some(title_part) = trimmed.strip_prefix("template-title:") {
                title = title_part.trim().to_string();
            } else if let Some(desc_part) = trimmed.strip_prefix("template-description:") {
                description = desc_part.trim().to_string();
            } else if trimmed == "template-requires-extended" {
                requires_extended = true;
            } else if trimmed == "template-interactive-mode" {
                interactive_mode = true;
            } else if trimmed == "template-content-start" {
                content_start = i + 1;
                break;
            }
        } else if !line.is_empty() {
            content_start = i;
            break;
        }
    }
    
    if title.is_empty() {
        return Ok(None);
    }
    
    let name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    let category = determine_template_category(path);
    let template_content = lines[content_start..].join("\n");
    
    Ok(Some(TemplateInfo {
        name,
        title,
        description,
        category,
        content: template_content,
        requires_extended,
        interactive_mode,
    }))
}

fn determine_template_category(path: &Path) -> String {
    if let Some(parent) = path.parent() {
        if let Some(dir_name) = parent.file_name().and_then(|s| s.to_str()) {
            return match dir_name {
                "basic" => "🎯 Basic Templates".to_string(),
                "arithmetic" => "🧮 Arithmetic Templates".to_string(),
                "extended" => "⚡ Extended Templates".to_string(),
                "io" => "💾 I/O Templates".to_string(),
                "arrays" => "📊 Array Templates".to_string(),
                "advanced" => "🔧 Advanced Templates".to_string(),
                _ => format!("📁 {}", dir_name),
            };
        }
    }
    "📄 Other".to_string()
}

fn categorize_templates(templates: Vec<TemplateInfo>) -> HashMap<String, Vec<TemplateInfo>> {
    let mut categorized: HashMap<String, Vec<TemplateInfo>> = HashMap::new();
    
    for template in templates {
        categorized.entry(template.category.clone())
            .or_insert_with(Vec::new)
            .push(template);
    }
    
    for templates in categorized.values_mut() {
        templates.sort_by(|a, b| a.name.cmp(&b.name));
    }
    
    categorized
}

fn generate_content_from_template(template: &TemplateInfo, project_name: &str) -> Result<String, AsmodeusError> {
    let content = template.content
        .replace("{{PROJECT_NAME}}", project_name)
        .replace("{{DATE}}", &chrono::Local::now().format("%Y-%m-%d").to_string());
    
    Ok(content)
}
