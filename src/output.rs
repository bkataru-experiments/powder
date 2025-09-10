use crate::themes::{Theme, ThemeManager};
use crate::history::HistoryEntry;
use colored::*;
use std::fs;
use std::path::Path;
use std::process;
use terminal_size::{Width, terminal_size};

pub struct OutputManager {
    pub theme_manager: ThemeManager,
    terminal_width: Option<usize>,
}

impl OutputManager {
    pub fn new() -> Self {
        let terminal_width = terminal_size().map(|(Width(w), _)| w as usize);
        
        Self {
            theme_manager: ThemeManager::new(),
            terminal_width,
        }
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), String> {
        self.theme_manager.set_theme(theme_name)
    }

    pub fn print_themes(&self, no_color: bool) {
        let themes = self.theme_manager.list_themes();
        let current_theme = self.theme_manager.get_current_theme();
        
        self.print_header("Available Themes", no_color);
        
        for theme in themes {
            let marker = if theme.name == current_theme.name { " (current)" } else { "" };
            let theme_line = format!("  {}: {}{}", 
                self.apply_theme_color(&theme.name, &current_theme.colors.info_label, no_color),
                theme.description,
                self.apply_theme_color(marker, &current_theme.colors.git_clean, no_color)
            );
            
            self.print_wrapped_line(&theme_line, no_color);
            
            // Show a sample of the theme colors
            let sample = format!("    Sample: {}{}{}{}{}",
                self.apply_color(&theme.colors.root_path, &theme.colors.root_path, no_color),
                self.apply_color("/", &theme.colors.separator, no_color),
                self.apply_color("path", &theme.colors.path_component, no_color),
                self.apply_color("/", &theme.colors.separator, no_color),
                self.apply_color("current", &theme.colors.current_directory, no_color)
            );
            
            self.print_wrapped_line(&sample, no_color);
            println!();
        }
    }

    pub fn print_history(&self, entries: Vec<&HistoryEntry>, no_color: bool) {
        if entries.is_empty() {
            let current_theme = self.theme_manager.get_current_theme();
            println!("{}", self.apply_theme_color("No directory history available.", 
                                                  &current_theme.colors.info_value, no_color));
            return;
        }

        self.print_header("Directory History", no_color);
        
        for (index, entry) in entries.iter().enumerate() {
            let current_theme = self.theme_manager.get_current_theme();
            let marker = if index == 0 { " (current)" } else { "" };
            
            // Format timestamp
            let datetime = format_timestamp(entry.timestamp);
            
            let history_line = format!("  {}: {}{} {}",
                self.apply_theme_color(&(index + 1).to_string(), &current_theme.colors.info_label, no_color),
                entry.path,
                self.apply_theme_color(marker, &current_theme.colors.git_clean, no_color),
                self.apply_theme_color(&format!("({})", datetime), &current_theme.colors.separator, no_color)
            );
            
            self.print_wrapped_line(&history_line, no_color);
        }
        println!();
    }

    pub fn print_verbose_info(&self, path: &str, no_color: bool) {
        let path_obj = Path::new(path);
        let current_theme = self.theme_manager.get_current_theme();
        
        self.print_header("Directory Information", no_color);
        
        println!("  {}: {}", 
            self.apply_theme_color("Path", &current_theme.colors.info_label, no_color),
            self.apply_theme_color(path, &current_theme.colors.info_value, no_color)
        );
        
        if let Some(parent) = path_obj.parent() {
            println!("  {}: {}", 
                self.apply_theme_color("Parent", &current_theme.colors.info_label, no_color),
                self.apply_theme_color(&parent.display().to_string(), &current_theme.colors.separator, no_color)
            );
        }
        
        if let Some(file_name) = path_obj.file_name() {
            println!("  {}: {}", 
                self.apply_theme_color("Current Folder", &current_theme.colors.info_label, no_color),
                self.apply_theme_color(&file_name.to_string_lossy(), &current_theme.colors.current_directory, no_color)
            );
        }
        
        // Check if it's a symlink
        if path_obj.is_symlink() {
            if let Ok(target) = path_obj.read_link() {
                println!("  {}: {}", 
                    self.apply_theme_color("Symlink Target", &current_theme.colors.info_label, no_color),
                    self.apply_theme_color(&target.display().to_string(), &current_theme.colors.git_branch, no_color)
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
                self.apply_theme_color("Type", &current_theme.colors.info_label, no_color),
                self.apply_theme_color(file_type, &current_theme.colors.info_value, no_color)
            );
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = metadata.permissions().mode();
                let permissions = format!("{:o}", mode & 0o777);
                println!("  {}: {}", 
                    self.apply_theme_color("Permissions", &current_theme.colors.info_label, no_color),
                    self.apply_theme_color(&permissions, &current_theme.colors.info_value, no_color)
                );
            }
        }
        
        println!(); // Empty line for spacing
    }

    pub fn print_git_info(&self, path: &str, no_color: bool) {
        let mut current_path = Path::new(path);
        let theme = self.theme_manager.get_current_theme();
        
        // Walk up the directory tree to find a .git directory
        loop {
            let git_path = current_path.join(".git");
            if git_path.exists() {
                self.print_header("Git Repository Information", no_color);
                
                println!("  {}: {}", 
                    self.apply_theme_color("Repository Root", &theme.colors.info_label, no_color),
                    self.apply_theme_color(&current_path.display().to_string(), &theme.colors.info_value, no_color)
                );
                
                // Try to read current branch
                let head_file = git_path.join("HEAD");
                if let Ok(head_content) = fs::read_to_string(head_file) {
                    if head_content.starts_with("ref: refs/heads/") {
                        let branch = head_content.trim().strip_prefix("ref: refs/heads/").unwrap_or("unknown");
                        println!("  {}: {}", 
                            self.apply_theme_color("Current Branch", &theme.colors.info_label, no_color),
                            self.apply_theme_color(branch, &theme.colors.git_branch, no_color)
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
                            self.apply_theme_color("Status", &theme.colors.info_label, no_color),
                            self.apply_theme_color("Has uncommitted changes", &theme.colors.git_dirty, no_color)
                        );
                    } else {
                        println!("  {}: {}", 
                            self.apply_theme_color("Status", &theme.colors.info_label, no_color),
                            self.apply_theme_color("Clean working tree", &theme.colors.git_clean, no_color)
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

    pub fn print_pretty_path(&self, path: &str, custom_separator: Option<&String>, no_color: bool) {
        let path_obj = Path::new(path);
        let components: Vec<_> = path_obj.components().collect();
        let theme = self.theme_manager.get_current_theme();
        
        if components.is_empty() {
            println!("{}", self.apply_theme_color("/", &theme.colors.root_path, no_color));
            return;
        }
        
        let separator = custom_separator.map(|s| s.as_str()).unwrap_or("/");
        let mut output = String::new();
        
        for (i, component) in components.iter().enumerate() {
            let component_str = component.as_os_str().to_string_lossy();
            
            // Skip root slash component if it's not the only component
            if component_str == "/" && components.len() > 1 {
                continue;
            }
            
            // Add separator before component (except for the first meaningful component)
            if i > 0 && !(i == 1 && components[0].as_os_str() == "/") {
                output.push_str(&self.apply_theme_color(separator, &theme.colors.separator, no_color));
            }
            
            // Handle root directory specially
            if component_str == "/" && components.len() == 1 {
                output.push_str(&self.apply_theme_color("/", &theme.colors.root_path, no_color));
            } else {
                // Color the component based on its position
                let color = match i {
                    _ if i == components.len() - 1 => &theme.colors.current_directory, // Current directory
                    0 if component_str != "/" => &theme.colors.root_path, // Drive root on Windows
                    _ => &theme.colors.path_component,
                };
                
                output.push_str(&self.apply_theme_color(&component_str, color, no_color));
            }
        }
        
        // Handle responsive output
        self.print_wrapped_line(&output, no_color);
    }

    pub fn print_error(&self, text: &str, no_color: bool) {
        let theme = self.theme_manager.get_current_theme();
        eprintln!("{}", self.apply_theme_color(text, &theme.colors.error, no_color));
    }

    fn print_header(&self, title: &str, no_color: bool) {
        let theme = self.theme_manager.get_current_theme();
        println!("{}", self.apply_theme_color(&format!("{}:", title), &theme.colors.info_label, no_color));
    }

    fn print_wrapped_line(&self, text: &str, _no_color: bool) {
        // For now, just print the line. In the future, we can add text wrapping based on terminal width
        if let Some(width) = self.terminal_width {
            if text.len() > width {
                // Simple word wrapping - split at word boundaries
                let words: Vec<&str> = text.split_whitespace().collect();
                let mut current_line = String::new();
                
                for word in words {
                    if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
                        println!("{}", current_line);
                        current_line = String::new();
                    }
                    
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
                
                if !current_line.is_empty() {
                    println!("{}", current_line);
                }
                return;
            }
        }
        
        println!("{}", text);
    }

    fn apply_theme_color(&self, text: &str, color_name: &str, no_color: bool) -> String {
        self.theme_manager.apply_color(text, color_name, no_color)
    }

    fn apply_color(&self, text: &str, color_name: &str, no_color: bool) -> String {
        self.theme_manager.apply_color(text, color_name, no_color)
    }
}

impl Default for OutputManager {
    fn default() -> Self {
        Self::new()
    }
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
        
    let elapsed = now.saturating_sub(timestamp);
    
    if elapsed < 60 {
        "just now".to_string()
    } else if elapsed < 3600 {
        format!("{}m ago", elapsed / 60)
    } else if elapsed < 86400 {
        format!("{}h ago", elapsed / 3600)
    } else {
        format!("{}d ago", elapsed / 86400)
    }
}