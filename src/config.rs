use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// Project config
///
/// located at `ciallo.toml`
#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub cmd: HashMap<String, Command>,
}

/// Global config
///
/// located at `~/.config/ciallo/config.toml`
#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    #[serde(default)]
    pub hook: HashMap<String, Hook>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Hook {
    Feishu(FeishuHook),
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeishuHook {
    pub webhook_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub command: String,
    #[serde(default = "default_stdout")]
    pub stdout: bool,
    #[serde(default = "default_stderr")]
    pub stderr: bool,
    #[serde(default)]
    pub hooks: Vec<String>,
}

fn default_stdout() -> bool {
    true
}

fn default_stderr() -> bool {
    true
}

impl ProjectConfig {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ProjectConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

impl GlobalConfig {
    pub fn load() -> anyhow::Result<Self> {
        let home = std::env::var("HOME")
            .map_err(|_| anyhow::anyhow!("HOME environment variable not set"))?;
        let config_path = PathBuf::from(home)
            .join(".config")
            .join("ciallo")
            .join("config.toml");

        if !config_path.exists() {
            return Ok(GlobalConfig {
                hook: HashMap::new(),
            });
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: GlobalConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
