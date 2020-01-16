use std::fs::read_to_string;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FedoraConfig {
    #[serde(rename(deserialize = "FAS"))]
    pub fas: FASConfig,
}

#[derive(Debug, Deserialize)]
pub struct FASConfig {
    pub username: String,
}

pub fn get_config() -> Result<FedoraConfig, String> {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => {
            return Err(String::from("Unable to determine $HOME."));
        },
    };

    let config_path = home.join(".config/fedora.toml");

    let config_str = match read_to_string(&config_path) {
        Ok(string) => string,
        Err(_) => {
            return Err(String::from(
                "Unable to read configuration file from ~/.config/fedora.toml",
            ));
        },
    };

    let config: FedoraConfig = match toml::from_str(&config_str) {
        Ok(config) => config,
        Err(_) => {
            return Err(String::from(
                "Unable to parse configuration file from ~/.config/fedora.toml",
            ));
        },
    };

    Ok(config)
}
