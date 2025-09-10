use crate::themes::ThemeManager;
use crate::history::HistoryEntry;
use std::fs;
use std::path::Path;
use std::process;
use terminal_size::{Width, Height, terminal_size};

pub struct OutputManager {
    pub theme_manager: ThemeManager,
    terminal_width: Option<usize>,
    terminal_height: Option<usize>,
}

impl OutputManager {
    pub fn new() -> Self {
        let terminal_size = terminal_size();
        let terminal_width = terminal_size.map(|(Width(w), _)| w as usize);
        let terminal_height = terminal_size.map(|(_, Height(h))| h as usize);
        
        Self {
            theme_manager: ThemeManager::new(),
            terminal_width,
            terminal_height,
        }
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), String> {
        self.theme_manager.set_theme(theme_name)
    }

    pub fn set_compact_mode(&mut self, compact: bool) {
        if compact {
            // Force compact mode regardless of terminal size
            self.terminal_width = Some(60);
            self.terminal_height = Some(15);
        }
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
        
        // Determine if we should show compact format based on terminal size
        let compact_format = self.terminal_width.map_or(false, |w| w < 100) || 
                            self.terminal_height.map_or(false, |h| h < 20);
        
        for (index, entry) in entries.iter().enumerate() {
            let current_theme = self.theme_manager.get_current_theme();
            let marker = if index == 0 { " (current)" } else { "" };
            
            if compact_format {
                // Compact format for small terminals
                let display_path = self.compress_path_for_display(&entry.path, 50);
                let history_line = format!("{}. {}{}", 
                    index + 1,
                    display_path,
                    self.apply_theme_color(marker, &current_theme.colors.git_clean, no_color)
                );
                self.print_wrapped_line(&history_line, no_color);
            } else {
                // Full format for larger terminals
                let datetime = format_timestamp(entry.timestamp);
                let history_line = format!("  {}: {}{} {}",
                    self.apply_theme_color(&(index + 1).to_string(), &current_theme.colors.info_label, no_color),
                    entry.path,
                    self.apply_theme_color(marker, &current_theme.colors.git_clean, no_color),
                    self.apply_theme_color(&format!("({})", datetime), &current_theme.colors.separator, no_color)
                );
                self.print_wrapped_line(&history_line, no_color);
            }
        }
        println!();
    }
    
    fn compress_path_for_display(&self, path: &str, max_length: usize) -> String {
        if path.len() <= max_length {
            return path.to_string();
        }
        
        let path_obj = Path::new(path);
        let components: Vec<_> = path_obj.components().collect();
        
        if components.len() <= 2 {
            // For short paths, just truncate
            let ellipsis = "…";
            let take_chars = max_length - ellipsis.len();
            return format!("{}{}", &path[..take_chars], ellipsis);
        }
        
        // For longer paths, show first, last, and compressed middle
        let first = components.first().map(|c| c.as_os_str().to_string_lossy()).unwrap_or_default();
        let last = components.last().map(|c| c.as_os_str().to_string_lossy()).unwrap_or_default();
        
        let separator = "/";
        let available_length = max_length - first.len() - last.len() - separator.len() * 2 - 3; // "…" takes 3
        
        if available_length > 0 {
            format!("{}{}{}{}{}", first, separator, "…", separator, last)
        } else {
            format!("…{}", last)
        }
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
        if let Some(git_root) = self.find_git_root(path) {
            let theme = self.theme_manager.get_current_theme();
            
            self.print_header("Git Repository Information", no_color);
            
            println!("  {}: {}", 
                self.apply_theme_color("Repository Root", &theme.colors.info_label, no_color),
                self.apply_theme_color(&git_root.display().to_string(), &theme.colors.info_value, no_color)
            );
            
            // Get branch info efficiently
            if let Some(branch) = self.get_git_branch(&git_root) {
                println!("  {}: {}", 
                    self.apply_theme_color("Current Branch", &theme.colors.info_label, no_color),
                    self.apply_theme_color(&branch, &theme.colors.git_branch, no_color)
                );
            }
            
            // Check git status efficiently
            match self.get_git_status(&git_root) {
                Some(true) => {
                    println!("  {}: {}", 
                        self.apply_theme_color("Status", &theme.colors.info_label, no_color),
                        self.apply_theme_color("Has uncommitted changes", &theme.colors.git_dirty, no_color)
                    );
                }
                Some(false) => {
                    println!("  {}: {}", 
                        self.apply_theme_color("Status", &theme.colors.info_label, no_color),
                        self.apply_theme_color("Clean working tree", &theme.colors.git_clean, no_color)
                    );
                }
                None => {
                    // Don't show status if we can't determine it quickly
                }
            }
            
            println!();
        }
    }
    
    fn find_git_root(&self, path: &str) -> Option<std::path::PathBuf> {
        let mut current_path = Path::new(path);
        
        // Walk up the directory tree to find a .git directory
        // Limit the search to avoid expensive traversals
        let max_depth = 20;
        for _ in 0..max_depth {
            let git_path = current_path.join(".git");
            if git_path.exists() {
                return Some(current_path.to_path_buf());
            }
            
            match current_path.parent() {
                Some(parent) => current_path = parent,
                None => break,
            }
        }
        None
    }
    
    fn get_git_branch(&self, git_root: &Path) -> Option<String> {
        let head_file = git_root.join(".git").join("HEAD");
        if let Ok(head_content) = fs::read_to_string(head_file) {
            if head_content.starts_with("ref: refs/heads/") {
                return head_content.trim().strip_prefix("ref: refs/heads/").map(|s| s.to_string());
            }
        }
        None
    }
    
    fn get_git_status(&self, git_root: &Path) -> Option<bool> {
        // Use a quick git status check with timeout to avoid hanging
        match process::Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(git_root)
            .output() {
            Ok(output) => Some(!output.stdout.is_empty()),
            Err(_) => None, // Git not available or error
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
        
        // Smart path compression for narrow terminals
        if let Some(width) = self.terminal_width {
            if width < 80 && components.len() > 3 {
                self.print_compressed_path(&components, separator, &theme, no_color, width);
                return;
            }
        }
        
        // Regular path display
        self.print_full_path(&components, separator, &theme, no_color);
    }
    
    fn print_compressed_path(&self, components: &[std::path::Component], separator: &str, 
                           theme: &crate::themes::Theme, no_color: bool, terminal_width: usize) {
        let mut output = String::new();
        let max_component_length = (terminal_width / components.len()).min(20).max(3);
        
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
            
            // Compress long components except the last one (current directory)
            let display_str = if i == components.len() - 1 {
                // Always show full current directory name
                component_str.to_string()
            } else if component_str.len() > max_component_length {
                // Compress middle components
                let ellipsis = "…";
                let take_chars = (max_component_length - ellipsis.len()).max(1);
                format!("{}{}", &component_str[..take_chars], ellipsis)
            } else {
                component_str.to_string()
            };
            
            // Color the component based on its position
            let color = match i {
                _ if i == components.len() - 1 => &theme.colors.current_directory,
                0 if component_str != "/" => &theme.colors.root_path,
                _ => &theme.colors.path_component,
            };
            
            output.push_str(&self.apply_theme_color(&display_str, color, no_color));
        }
        
        println!("{}", output);
    }
    
    fn print_full_path(&self, components: &[std::path::Component], separator: &str, 
                       theme: &crate::themes::Theme, no_color: bool) {
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
    use std::time::{SystemTime, UNIX_EPOCH};
    
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