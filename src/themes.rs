use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub root_path: String,
    pub path_component: String,
    pub current_directory: String,
    pub separator: String,
    pub git_branch: String,
    pub git_clean: String,
    pub git_dirty: String,
    pub info_label: String,
    pub info_value: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub colors: ThemeColors,
}

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        
        // Default theme (current color scheme)
        themes.insert("default".to_string(), Theme {
            name: "default".to_string(),
            description: "Classic powder color scheme with bright colors".to_string(),
            colors: ThemeColors {
                root_path: "bright_blue".to_string(),
                path_component: "bright_white".to_string(),
                current_directory: "bright_yellow".to_string(),
                separator: "bright_black".to_string(),
                git_branch: "bright_yellow".to_string(),
                git_clean: "bright_green".to_string(),
                git_dirty: "bright_red".to_string(),
                info_label: "green".to_string(),
                info_value: "bright_white".to_string(),
                error: "red".to_string(),
            },
        });

        // Ocean theme
        themes.insert("ocean".to_string(), Theme {
            name: "ocean".to_string(),
            description: "Calming blue-green oceanic color palette".to_string(),
            colors: ThemeColors {
                root_path: "blue".to_string(),
                path_component: "cyan".to_string(),
                current_directory: "bright_cyan".to_string(),
                separator: "blue".to_string(),
                git_branch: "bright_blue".to_string(),
                git_clean: "green".to_string(),
                git_dirty: "bright_red".to_string(),
                info_label: "blue".to_string(),
                info_value: "bright_cyan".to_string(),
                error: "red".to_string(),
            },
        });

        // Forest theme
        themes.insert("forest".to_string(), Theme {
            name: "forest".to_string(),
            description: "Natural green forest-inspired colors".to_string(),
            colors: ThemeColors {
                root_path: "green".to_string(),
                path_component: "bright_green".to_string(),
                current_directory: "bright_yellow".to_string(),
                separator: "green".to_string(),
                git_branch: "yellow".to_string(),
                git_clean: "bright_green".to_string(),
                git_dirty: "red".to_string(),
                info_label: "green".to_string(),
                info_value: "bright_white".to_string(),
                error: "red".to_string(),
            },
        });

        // Sunset theme
        themes.insert("sunset".to_string(), Theme {
            name: "sunset".to_string(),
            description: "Warm sunset colors with oranges and reds".to_string(),
            colors: ThemeColors {
                root_path: "red".to_string(),
                path_component: "yellow".to_string(),
                current_directory: "bright_red".to_string(),
                separator: "red".to_string(),
                git_branch: "bright_yellow".to_string(),
                git_clean: "green".to_string(),
                git_dirty: "bright_red".to_string(),
                info_label: "red".to_string(),
                info_value: "bright_yellow".to_string(),
                error: "bright_red".to_string(),
            },
        });

        // Monochrome theme
        themes.insert("mono".to_string(), Theme {
            name: "mono".to_string(),
            description: "Minimalist monochrome grayscale theme".to_string(),
            colors: ThemeColors {
                root_path: "bright_white".to_string(),
                path_component: "white".to_string(),
                current_directory: "bright_white".to_string(),
                separator: "bright_black".to_string(),
                git_branch: "white".to_string(),
                git_clean: "white".to_string(),
                git_dirty: "bright_white".to_string(),
                info_label: "white".to_string(),
                info_value: "bright_white".to_string(),
                error: "white".to_string(),
            },
        });

        Self {
            themes,
            current_theme: "default".to_string(),
        }
    }

    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    pub fn get_current_theme(&self) -> &Theme {
        self.themes.get(&self.current_theme).unwrap()
    }

    pub fn set_theme(&mut self, name: &str) -> Result<(), String> {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    pub fn list_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }

    pub fn apply_color(&self, text: &str, color_name: &str, no_color: bool) -> String {
        if no_color {
            return text.to_string();
        }

        let colored_text = match color_name {
            "red" => text.red(),
            "green" => text.green(),
            "yellow" => text.yellow(),
            "blue" => text.blue(),
            "magenta" => text.magenta(),
            "cyan" => text.cyan(),
            "white" => text.white(),
            "black" => text.black(),
            "bright_red" => text.bright_red(),
            "bright_green" => text.bright_green(),
            "bright_yellow" => text.bright_yellow(),
            "bright_blue" => text.bright_blue(),
            "bright_magenta" => text.bright_magenta(),
            "bright_cyan" => text.bright_cyan(),
            "bright_white" => text.bright_white(),
            "bright_black" => text.bright_black(),
            _ => text.normal(),
        };
        colored_text.to_string()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}