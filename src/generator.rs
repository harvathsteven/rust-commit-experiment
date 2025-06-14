use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::git::GitAnalyzer;
use crate::personalities::get_personality;

#[derive(Serialize, Deserialize, Default)]
pub struct Favorites {
    messages: HashMap<String, Vec<String>>,
}

pub struct CommitGenerator {
    favorites: Favorites,
    favorites_path: PathBuf,
}

impl CommitGenerator {
    pub fn new() -> Result<Self> {
        let favorites_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".git-commit-ai")
            .join("favorites.json");

        let favorites = if favorites_path.exists() {
            let content = fs::read_to_string(&favorites_path)?;
            serde_json::from_str(&content)?
        } else {
            Favorites::default()
        };

        Ok(Self {
            favorites,
            favorites_path,
        })
    }

    pub fn generate_commit_message(&self, personality_name: &str) -> Result<String> {
        let git_analyzer = GitAnalyzer::new()?;
        let analysis = git_analyzer.analyze_changes()?;

        let personality = get_personality(personality_name)
            .ok_or_else(|| anyhow::anyhow!("Invalid personality: {}", personality_name))?;

        Ok(personality.generate_message(&analysis))
    }

    pub fn save_favorite(&mut self, personality: &str, message: &str) -> Result<()> {
        let messages = self.favorites.messages
            .entry(personality.to_string())
            .or_insert_with(Vec::new);
        
        if !messages.contains(&message.to_string()) {
            messages.push(message.to_string());
            self.save_favorites()?;
        }

        Ok(())
    }

    fn save_favorites(&self) -> Result<()> {
        if let Some(parent) = self.favorites_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(&self.favorites)?;
        fs::write(&self.favorites_path, content)?;
        
        Ok(())
    }
} 