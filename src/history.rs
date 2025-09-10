use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub path: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    entries: VecDeque<HistoryEntry>,
    max_entries: usize,
}

impl History {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    pub fn add_entry(&mut self, path: String) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let entry = HistoryEntry { path, timestamp };

        // Remove duplicate if it exists (move to front)
        self.entries.retain(|e| e.path != entry.path);

        // Add to front
        self.entries.push_front(entry);

        // Keep only max_entries
        while self.entries.len() > self.max_entries {
            self.entries.pop_back();
        }
    }

    pub fn get_entries(&self) -> Vec<&HistoryEntry> {
        self.entries.iter().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

pub struct HistoryManager {
    history: History,
    history_file: PathBuf,
}

impl HistoryManager {
    pub fn new() -> Result<Self, String> {
        let config_dir = get_config_dir()?;
        let history_file = config_dir.join(".powder.history");

        let history = if history_file.exists() {
            Self::load_history(&history_file)?
        } else {
            History::new(10)
        };

        Ok(Self {
            history,
            history_file,
        })
    }

    pub fn add_current_directory(&mut self, path: String) -> Result<(), String> {
        self.history.add_entry(path);
        self.save_history()
    }

    pub fn get_history(&self) -> Vec<&HistoryEntry> {
        self.history.get_entries()
    }

    fn load_history(file_path: &Path) -> Result<History, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read history file: {}", e))?;

        let history: History = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse history file: {}", e))?;

        Ok(history)
    }

    fn save_history(&self) -> Result<(), String> {
        // Ensure directory exists
        if let Some(parent) = self.history_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(&self.history)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;

        fs::write(&self.history_file, content)
            .map_err(|e| format!("Failed to write history file: {}", e))?;

        Ok(())
    }
}

fn get_config_dir() -> Result<PathBuf, String> {
    // Try to get user's home directory first
    if let Some(home_dir) = dirs::home_dir() {
        return Ok(home_dir);
    }

    // Fallback to config directory
    if let Some(config_dir) = dirs::config_dir() {
        let powder_config = config_dir.join("powder");
        return Ok(powder_config);
    }

    // Last resort - current directory
    std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory for config: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_add_entry() {
        let mut history = History::new(3);
        
        history.add_entry("/path/1".to_string());
        history.add_entry("/path/2".to_string());
        history.add_entry("/path/3".to_string());
        
        assert_eq!(history.len(), 3);
        assert_eq!(history.get_entries()[0].path, "/path/3");
        assert_eq!(history.get_entries()[1].path, "/path/2");
        assert_eq!(history.get_entries()[2].path, "/path/1");
    }

    #[test]
    fn test_history_max_entries() {
        let mut history = History::new(2);
        
        history.add_entry("/path/1".to_string());
        history.add_entry("/path/2".to_string());
        history.add_entry("/path/3".to_string());
        
        assert_eq!(history.len(), 2);
        assert_eq!(history.get_entries()[0].path, "/path/3");
        assert_eq!(history.get_entries()[1].path, "/path/2");
    }

    #[test]
    fn test_history_duplicate_removal() {
        let mut history = History::new(5);
        
        history.add_entry("/path/1".to_string());
        history.add_entry("/path/2".to_string());
        history.add_entry("/path/1".to_string()); // Duplicate
        
        assert_eq!(history.len(), 2);
        assert_eq!(history.get_entries()[0].path, "/path/1");
        assert_eq!(history.get_entries()[1].path, "/path/2");
    }
}