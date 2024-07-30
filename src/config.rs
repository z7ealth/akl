use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

const CONFIGURATION_PATH: &str = "/etc/akl/config.toml";

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

pub fn get_config() -> Result<AKLConfig, ()> {
    let path = Path::new(CONFIGURATION_PATH);

    if !path.exists() {
        return Ok(AKLConfig::default());
    }

    let contents = read_to_string(path).unwrap();

    match toml::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(msg) => panic!("Unable to read AKL configuration file: {}", msg),
    }
}
