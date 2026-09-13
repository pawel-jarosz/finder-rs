use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    default_collection: String,
    current_collection: String,
    collections: HashMap<String, Collection>,
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    path: PathBuf,
    description: String,
}

pub enum ConfigurationStatus {
    ParseError,
    Success(Configuration)
}

impl Configuration {
    pub fn load(content: &str) -> ConfigurationStatus {
        let configuration: Result<Configuration, serde_yaml::Error> = serde_yaml::from_str(&content);

        match configuration {
            Ok(config) => ConfigurationStatus::Success(config),
            Err(_) => ConfigurationStatus::ParseError,
        }
    }
}

pub struct ConfigurationHandler {
    pub configuration: Configuration,
    pub changed: bool,
}

impl ConfigurationHandler {
    pub(crate) fn load_configuration(content: &str) -> Result<ConfigurationHandler, ConfigurationStatus> {
        match Configuration::load(content) {
            ConfigurationStatus::Success(config) => Ok(ConfigurationHandler {
                configuration: config,
                changed: false,
            }),
            ConfigurationStatus::ParseError => Err(ConfigurationStatus::ParseError),
        }
    }
}

pub fn default_configuration() -> &'static str {
    r#"
default_collection: default
current_collection: default
collections:
  default:
    path: default
    description: Default collection
    "#
}
