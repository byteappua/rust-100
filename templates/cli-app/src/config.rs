use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 应用配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// 应用名称
    pub app_name: String,

    /// 版本
    pub version: String,

    /// 其他配置项
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    /// 是否启用详细输出
    pub verbose: bool,

    /// 最大并发数
    pub max_concurrency: usize,

    /// 超时时间(秒)
    pub timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "CLI App".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            settings: Settings {
                verbose: false,
                max_concurrency: 4,
                timeout: 30,
            },
        }
    }
}

impl Config {
    /// 从文件加载配置
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            log::warn!("Config file not found, using default config");
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;

        Ok(config)
    }

    /// 保存配置到文件
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write config file: {:?}", path.as_ref()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.app_name, "CLI App");
        assert!(!config.settings.verbose);
    }

    #[test]
    fn test_save_and_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = Config::default();

        config.save(temp_file.path()).unwrap();
        let loaded = Config::load(temp_file.path()).unwrap();

        assert_eq!(config.app_name, loaded.app_name);
    }
}
