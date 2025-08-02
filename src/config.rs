use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::python_env::PythonEnv;

#[derive(Deserialize, Serialize, Debug)]
pub struct PythonConfig {
    pub python_exe: String,
    pub venv_path: String,
    pub requirements_file: String,
}

impl Default for PythonConfig {
    fn default() -> Self {
        Self {
            python_exe: "python".to_string(),
            venv_path: "./venv".to_string(),
            requirements_file: "requirements.txt".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub app_name: String,
    pub database_url: String,
    pub python: PythonConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "Hello world!".to_string(),
            database_url: "sqlite:app.db".to_string(),
            python: PythonConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Create default config
                let default = Config::default();
                let toml_str = toml::to_string_pretty(&default)?;
                fs::write(path, toml_str)?;
                return Ok(default);
            }
            Err(e) => return Err(anyhow::anyhow!("Failed to read config: {}", e)),
        };
        
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn create_python_env(&self) -> Result<PythonEnv> {
        PythonEnv::new_with_config(&self.python)
    }
}