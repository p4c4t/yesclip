use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::{fs, io::Write, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub copy_text_files_as_plain: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { copy_text_files_as_plain: false }
    }
}

impl Settings {
    pub fn path() -> PathBuf {
        config_dir().unwrap().join("yesclip").join("settings.json")
    }
    
    pub fn load() -> Self {
        let path = Self::path();
        fs::create_dir_all(path.parent().unwrap()).ok();
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
    
    pub fn save(&self) -> anyhow::Result<()> {
        let s = serde_json::to_string_pretty(self)?;
        let mut f = fs::File::create(Self::path())?;
        f.write_all(s.as_bytes())?;
        Ok(())
    }
}

