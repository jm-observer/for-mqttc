use anyhow::Result;
use log::error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub debug: bool,
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
        if let Ok(config) = Self::_init(file_path.clone()) {
            config
        } else {
            let config = Self::default();
            if let Err(e) = config.clone()._update(file_path) {
                error!("config update fail: {:?}", e);
            }
            config
        }
    }

    fn _init(file_path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(content.as_str())?;
        Ok(config)
    }

    fn _update(self, file_path: PathBuf) -> Result<()> {
        std::fs::write(file_path, serde_json::to_string(&self)?)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { debug: false }
    }
}

impl Default for AutoRetract {
    fn default() -> Self {
        Self::Open(30)
    }
}
