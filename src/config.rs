use crate::error::{JdkError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JdkInfo {
    pub path: PathBuf,
    pub version: String,
    pub vendor: Option<String>,
    pub java_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub current_jdk: Option<String>,
    pub jdks: HashMap<String, JdkInfo>,
    pub download_dir: PathBuf,
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let exe_path = std::env::current_exe()
            .map_err(|e| JdkError::ConfigError(format!("Cannot get executable path: {}", e)))?;
        
        let exe_dir = exe_path.parent()
            .ok_or_else(|| JdkError::ConfigError("Cannot get executable directory".to_string()))?;
        
        Ok(exe_dir.to_path_buf())
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| JdkError::ConfigError(format!("Failed to read config: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| JdkError::ConfigError(format!("Failed to parse config: {}", e)))
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()?;
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .map_err(|e| JdkError::ConfigError(format!("Failed to create config dir: {}", e)))?;
        }

        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| JdkError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        fs::write(&path, content)
            .map_err(|e| JdkError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    pub fn add_jdk(&mut self, key: String, info: JdkInfo) {
        self.jdks.insert(key, info);
    }

    pub fn get_jdk(&self, key: &str) -> Option<&JdkInfo> {
        self.jdks.get(key)
    }

    pub fn set_current(&mut self, key: String) {
        self.current_jdk = Some(key);
    }

    pub fn get_current(&self) -> Option<&JdkInfo> {
        self.current_jdk.as_ref().and_then(|k| self.jdks.get(k))
    }
}

impl Default for Config {
    fn default() -> Self {
        let download_dir = Self::config_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("downloads");

        Self {
            current_jdk: None,
            jdks: HashMap::new(),
            download_dir,
        }
    }
}
