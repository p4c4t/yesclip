use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub copy_text_files_as_plain: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            copy_text_files_as_plain: true,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        match std::fs::read_to_string(&config_path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(settings) => settings,
                Err(_) => {
                    let default = Self::default();
                    let _ = default.save();
                    default
                }
            },
            Err(_) => {
                let default = Self::default();
                let _ = default.save();
                default
            }
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, contents)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("yesclip")
            .join("config.json")
    }
}