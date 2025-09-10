mod themes;
mod history;
mod output;

use clap::{Arg, Command};
use std::env;
use std::path::Path;
use std::process;

use crate::history::HistoryManager;
use crate::output::OutputManager;

fn main() {
    let matches = Command::new("powder")
        .version("0.1.0")
        .author("bkataru")
        .about("A sophisticated replica of the pwd command with pretty printing")
        .long_about("powder (or pd) is a modern, colorful alternative to pwd. It displays the current working directory with beautiful formatting, colors, themes, and directory history tracking.")
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
        .arg(
            Arg::new("theme")
                .short('t')
                .long("theme")
                .value_name("THEME")
                .help("Set the color theme for output (default, ocean, forest, sunset, mono)")
                .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("themes")
                .long("themes")
                .help("List all available themes with descriptions and samples")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("history")
                .long("history")
                .help("Show the past 10 working directories (most recent first)")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let use_logical = matches.get_flag("logical");
    let use_physical = matches.get_flag("physical");
    let verbose = matches.get_flag("verbose");
    let show_git = matches.get_flag("git");
    let no_color = matches.get_flag("no-color");
    let list_themes = matches.get_flag("themes");
    let show_history = matches.get_flag("history");
    let custom_separator = matches.get_one::<String>("separator");
    let theme_name = matches.get_one::<String>("theme");

    // Handle conflicting options
    if use_logical && use_physical {
        eprintln!("Error: --logical and --physical options are mutually exclusive");
        process::exit(1);
    }

    // Initialize output manager
    let mut output_manager = OutputManager::new();
    
    // Set theme if specified
    if let Some(theme) = theme_name {
        if let Err(e) = output_manager.set_theme(theme) {
            output_manager.print_error(&e, no_color);
            process::exit(1);
        }
    }

    // Handle list themes option
    if list_themes {
        output_manager.print_themes(no_color);
        return;
    }

    // Handle history option
    if show_history {
        match HistoryManager::new() {
            Ok(history_manager) => {
                let entries = history_manager.get_history();
                output_manager.print_history(entries, no_color);
            }
            Err(e) => {
                output_manager.print_error(&format!("Failed to load history: {}", e), no_color);
            }
        }
        return;
    }

    let current_dir = get_current_directory(use_logical, use_physical);
    
    match current_dir {
        Ok(path) => {
            // Add to history (ignore errors for non-critical functionality)
            if let Ok(mut history_manager) = HistoryManager::new() {
                let _ = history_manager.add_current_directory(path.clone());
            }
            
            if verbose {
                output_manager.print_verbose_info(&path, no_color);
            }
            
            if show_git {
                output_manager.print_git_info(&path, no_color);
            }
            
            output_manager.print_pretty_path(&path, custom_separator, no_color);
        }
        Err(e) => {
            output_manager.print_error(&format!("Error getting current directory: {}", e), no_color);
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


