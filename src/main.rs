use clap::{Arg, Command};
use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let matches = Command::new("powder")
        .version("0.1.0")
        .author("bkataru")
        .about("A sophisticated replica of the pwd command with pretty printing")
        .long_about("powder (or pd) is a modern, colorful alternative to pwd. It displays the current working directory with beautiful formatting and colors.")
        .arg(
            Arg::new("logical")
                .short('L')
                .long("logical")
                .help("Use PWD from environment, even if it contains symlinks")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("physical")
                .short('P')
                .long("physical")
                .help("Avoid all symlinks")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output with detailed directory information")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("separator")
                .short('s')
                .long("separator")
                .value_name("SEP")
                .help("Use custom path separator for display")
                .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("git")
                .short('g')
                .long("git")
                .help("Show git repository information if available")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Disable colored output")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let use_logical = matches.get_flag("logical");
    let use_physical = matches.get_flag("physical");
    let verbose = matches.get_flag("verbose");
    let show_git = matches.get_flag("git");
    let no_color = matches.get_flag("no-color");
    let custom_separator = matches.get_one::<String>("separator");

    // Handle conflicting options
    if use_logical && use_physical {
        eprintln!("{}", format_error("Error: --logical and --physical options are mutually exclusive", no_color));
        process::exit(1);
    }

    let current_dir = get_current_directory(use_logical, use_physical);
    
    match current_dir {
        Ok(path) => {
            if verbose {
                print_verbose_info(&path, no_color);
            }
            
            if show_git {
                print_git_info(&path, no_color);
            }
            
            print_pretty_path(&path, custom_separator, no_color);
        }
        Err(e) => {
            eprintln!("{}: {}", format_error("Error getting current directory", no_color), e);
            process::exit(1);
        }
    }
}

fn get_current_directory(use_logical: bool, use_physical: bool) -> Result<String, String> {
    if use_logical {
        // Try to get PWD from environment first
        if let Ok(pwd) = env::var("PWD") {
            return Ok(pwd);
        }
    }
    
    // Default behavior or when --physical is specified
    match env::current_dir() {
        Ok(path) => {
            if use_physical {
                // Canonicalize to resolve all symlinks
                match path.canonicalize() {
                    Ok(canonical_path) => Ok(canonical_path.to_string_lossy().to_string()),
                    Err(e) => Err(format!("Failed to canonicalize path: {}", e)),
                }
            } else {
                Ok(path.to_string_lossy().to_string())
            }
        }
        Err(e) => Err(format!("Failed to get current directory: {}", e)),
    }
}

fn print_verbose_info(path: &str, no_color: bool) {
    let path_obj = Path::new(path);
    
    println!("{}", format_text("Directory Information:", no_color, |s| s.cyan().bold()));
    println!("  {}: {}", 
        format_text("Path", no_color, |s| s.green()), 
        format_text(path, no_color, |s| s.bright_white())
    );
    
    if let Some(parent) = path_obj.parent() {
        println!("  {}: {}", 
            format_text("Parent", no_color, |s| s.green()), 
            format_text(&parent.display().to_string(), no_color, |s| s.bright_black())
        );
    }
    
    if let Some(file_name) = path_obj.file_name() {
        println!("  {}: {}", 
            format_text("Current Folder", no_color, |s| s.green()), 
            format_text(&file_name.to_string_lossy(), no_color, |s| s.bright_yellow())
        );
    }
    
    // Check if it's a symlink
    if path_obj.is_symlink() {
        if let Ok(target) = path_obj.read_link() {
            println!("  {}: {}", 
                format_text("Symlink Target", no_color, |s| s.green()), 
                format_text(&target.display().to_string(), no_color, |s| s.bright_magenta())
            );
        }
    }
    
    // Show file permissions and type
    if let Ok(metadata) = path_obj.metadata() {
        let file_type = if metadata.is_dir() { "Directory" } 
                       else if metadata.is_file() { "File" }
                       else if metadata.is_symlink() { "Symlink" }
                       else { "Other" };
        println!("  {}: {}", 
            format_text("Type", no_color, |s| s.green()), 
            format_text(file_type, no_color, |s| s.bright_cyan())
        );
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            let permissions = format!("{:o}", mode & 0o777);
            println!("  {}: {}", 
                format_text("Permissions", no_color, |s| s.green()), 
                format_text(&permissions, no_color, |s| s.bright_white())
            );
        }
    }
    
    println!(); // Empty line for spacing
}

fn print_git_info(path: &str, no_color: bool) {
    let mut current_path = Path::new(path);
    
    // Walk up the directory tree to find a .git directory
    loop {
        let git_path = current_path.join(".git");
        if git_path.exists() {
            println!("{}", format_text("Git Repository Information:", no_color, |s| s.magenta().bold()));
            println!("  {}: {}", 
                format_text("Repository Root", no_color, |s| s.green()), 
                format_text(&current_path.display().to_string(), no_color, |s| s.bright_white())
            );
            
            // Try to read current branch
            let head_file = git_path.join("HEAD");
            if let Ok(head_content) = fs::read_to_string(head_file) {
                if head_content.starts_with("ref: refs/heads/") {
                    let branch = head_content.trim().strip_prefix("ref: refs/heads/").unwrap_or("unknown");
                    println!("  {}: {}", 
                        format_text("Current Branch", no_color, |s| s.green()), 
                        format_text(branch, no_color, |s| s.bright_yellow())
                    );
                }
            }
            
            // Check if there are uncommitted changes
            if let Ok(status_output) = process::Command::new("git")
                .args(["status", "--porcelain"])
                .current_dir(current_path)
                .output() {
                if !status_output.stdout.is_empty() {
                    println!("  {}: {}", 
                        format_text("Status", no_color, |s| s.green()), 
                        format_text("Has uncommitted changes", no_color, |s| s.bright_red())
                    );
                } else {
                    println!("  {}: {}", 
                        format_text("Status", no_color, |s| s.green()), 
                        format_text("Clean working tree", no_color, |s| s.bright_green())
                    );
                }
            }
            
            println!(); // Empty line for spacing
            break;
        }
        
        match current_path.parent() {
            Some(parent) => current_path = parent,
            None => break, // Reached filesystem root
        }
    }
}

fn print_pretty_path(path: &str, custom_separator: Option<&String>, no_color: bool) {
    let path_obj = Path::new(path);
    let components: Vec<_> = path_obj.components().collect();
    
    if components.is_empty() {
        println!("{}", format_text("/", no_color, |s| s.bright_blue().bold()));
        return;
    }
    
    let separator = custom_separator.map(|s| s.as_str()).unwrap_or("/");
    
    for (i, component) in components.iter().enumerate() {
        let component_str = component.as_os_str().to_string_lossy();
        
        // Skip root slash component if it's not the only component
        if component_str == "/" && components.len() > 1 {
            continue;
        }
        
        // Add separator before component (except for the first meaningful component)
        if i > 0 && !(i == 1 && components[0].as_os_str() == "/") {
            print!("{}", format_text(separator, no_color, |s| s.bright_black()));
        }
        
        // Handle root directory specially
        if component_str == "/" && components.len() == 1 {
            print!("{}", format_text("/", no_color, |s| s.bright_blue().bold()));
        } else {
            // Color the component based on its position
            let formatted_component = match i {
                _ if i == components.len() - 1 => format_text(&component_str, no_color, |s| s.bright_yellow().bold()), // Current directory
                0 if component_str != "/" => format_text(&component_str, no_color, |s| s.bright_blue().bold()), // Drive root on Windows
                _ => format_text(&component_str, no_color, |s| s.bright_white()),
            };
            
            print!("{}", formatted_component);
        }
    }
    
    println!(); // New line at the end
}

fn format_text<F>(text: &str, no_color: bool, color_fn: F) -> String
where
    F: FnOnce(ColoredString) -> ColoredString,
{
    if no_color {
        text.to_string()
    } else {
        color_fn(text.normal()).to_string()
    }
}

fn format_error(text: &str, no_color: bool) -> String {
    format_text(text, no_color, |s| s.red())
}
