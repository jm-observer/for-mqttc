use anyhow::Result;
use log::error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub display_tips: bool,
    pub theme: Theme,
    pub payload_font_size: f64,
    pub auto_retract: AutoRetract,
}
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum AutoRetract {
    Close,
    Open(u64),
}

impl Config {
    pub fn init(home_path: PathBuf) -> Self {
        let file_path = home_path.join("config.json");
        if let Ok(config) = Self::_init(file_path) {
            config
        } else {
            Self::default()
        }
    }

    fn _init(file_path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(content.as_str())?;
        Ok(config)
    }

    pub fn is_ligth(&self) -> bool {
        self.theme == Theme::Light
    }

    pub fn update(self, home_path: PathBuf) {
        let file_path = home_path.join("config.json");
        if let Err(e) = self._update(file_path) {
            error!("update config fail: {:?}", e);
        };
    }

    fn _update(self, file_path: PathBuf) -> Result<()> {
        std::fs::write(file_path, serde_json::to_string(&self)?)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display_tips: true,
            theme: Theme::Light,
            payload_font_size: 14.0,
            auto_retract: Default::default(),
        }
    }
}

impl Default for AutoRetract {
    fn default() -> Self {
        Self::Open(30)
    }
}
