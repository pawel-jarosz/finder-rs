use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    default_collection: String,
    pub(crate) current_collection: String,
    collections: HashMap<String, Collection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub(crate) path: PathBuf,
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
    pub configuration_path: PathBuf,
    pub changed: bool,
}

impl ConfigurationHandler {
    pub(crate) fn load_configuration(content: &str, config_name: PathBuf) -> Result<ConfigurationHandler, ConfigurationStatus> {
        match Configuration::load(content) {
            ConfigurationStatus::Success(config) => Ok(ConfigurationHandler {
                configuration: config,
                configuration_path: config_name,
                changed: false,
            }),
            ConfigurationStatus::ParseError => Err(ConfigurationStatus::ParseError),
        }
    }

    pub(crate) fn list_collections(&self) {
        for item in self.configuration.collections.iter() {
            let state = if item.0.as_str() == self.configuration.current_collection {
                "active"
            } else {
                ""
            };
            println!("{:15} | {:10} | {:40} | {}", item.0, state, item.1.description, item.1.path.display());
        }
    }

    pub(crate) fn activate_collection(&mut self, collection_name: String) {
        self.configuration.current_collection = collection_name;
        self.changed = true;
    }

    pub(crate) fn current_collection(&self) -> &str {
        &self.configuration.current_collection
    }

    pub(crate) fn get_collection(&self, collection_name: &str) -> Option<&Collection> {
        self.configuration.collections.get(collection_name)
    }

    pub fn save(&self) {
        let content = serde_yaml::to_string(&self.configuration).unwrap();
        std::fs::write(&self.configuration_path, content).expect("File cannot be overwritten");
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
