use serde::Deserialize;
use std::path::PathBuf;
use anyhow::{Result, Context};
use config::ConfigError;

#[derive(Debug, Deserialize)]
pub struct CliConfig {
    #[serde(default = "default_device_name")]
    pub device_name: String,
    #[serde(default = "default_discovery_port")]
    pub discovery_port: u16,
    #[serde(default = "default_transfer_port")]
    pub transfer_port: u16,
    #[serde(default = "default_download_dir")]
    pub download_dir: PathBuf,
}

fn default_device_name() -> String {
    gethostname::gethostname().to_string_lossy().into_owned()
}

fn default_discovery_port() -> u16 {
    5353
}

fn default_transfer_port() -> u16 {
    5354
}

fn default_download_dir() -> PathBuf {
    PathBuf::from("./downloads")
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            device_name: default_device_name(),
            discovery_port: default_discovery_port(),
            transfer_port: default_transfer_port(),
            download_dir: default_download_dir(),
        }
    }
}

impl CliConfig {
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("TSUNAGU"))
            .build()
            .context("Failed to build configuration")?;

        config.try_deserialize().map_err(|e| {
            match e {
                ConfigError::Message(msg) if msg.contains("missing field") => {
                    anyhow::anyhow!("Configuration error: Some fields are missing. Using default values where possible. Details: {}", msg)
                },
                _ => anyhow::anyhow!("Failed to parse configuration: {}", e),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = CliConfig::default();
        assert!(!config.device_name.is_empty());
        assert_eq!(config.discovery_port, 5353);
        assert_eq!(config.transfer_port, 5354);
        assert!(!config.download_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_load_config_from_env() {
        env::set_var("TSUNAGU_DEVICE_NAME", "TestDevice");
        env::set_var("TSUNAGU_DISCOVERY_PORT", "8000");
        env::set_var("TSUNAGU_TRANSFER_PORT", "8001");
        env::set_var("TSUNAGU_DOWNLOAD_DIR", "/tmp/downloads");

        let config = CliConfig::load().unwrap();

        assert_eq!(config.device_name, "TestDevice");
        assert_eq!(config.discovery_port, 8000);
        assert_eq!(config.transfer_port, 8001);
        assert_eq!(config.download_dir, PathBuf::from("/tmp/downloads"));

        // Clean up environment variables
        env::remove_var("TSUNAGU_DEVICE_NAME");
        env::remove_var("TSUNAGU_DISCOVERY_PORT");
        env::remove_var("TSUNAGU_TRANSFER_PORT");
        env::remove_var("TSUNAGU_DOWNLOAD_DIR");
    }
}
