use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::Deserialize;

const SYSTEM_CONFIGURATION_PATH: &str = "/etc/akl/config.toml";

#[derive(Deserialize)]
pub struct AKLConfig {
    pub product: String,
    pub mode: String,
}

impl Default for AKLConfig {
    fn default() -> Self {
        AKLConfig {
            product: "AK500".to_string(),
            mode: "temp".to_string(),
        }
    }
}

fn get_config_path() -> PathBuf {
    // 1. Explicit env override
    if let Ok(config_dir) = env::var("AKL_CONFIG_DIR") {
        return Path::new(&config_dir).join("config.toml");
    }

    // 2. User config directory
    if let Ok(home) = env::var("HOME") {
        let user_config =
            Path::new(&home).join(".config/akl/config.toml");

        if user_config.exists() {
            return user_config;
        }
    }

    // 3. System-wide fallback
    PathBuf::from(SYSTEM_CONFIGURATION_PATH)
}

pub fn get_config() -> Result<AKLConfig, ()> {
    let path = get_config_path();

    if !path.exists() {
        return Ok(AKLConfig::default());
    }

    let contents = match read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!(
                "Unable to read configuration file {}: {}",
                path.display(),
                err
            );

            return Ok(AKLConfig::default());
        }
    };

    match toml::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => {
            eprintln!(
                "Unable to parse configuration file {}: {}",
                path.display(),
                err
            );

            Ok(AKLConfig::default())
        }
    }
}
