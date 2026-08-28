use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub path: String,
    pub description: String
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub default_collection: String,
    pub cache_file: String,
    collections: HashMap<String, Collection>,
}

impl Configuration {
    pub fn load_from_json(content: &str) -> serde_json::Result<Configuration> {
        serde_json::from_str::<Configuration>(content)
    }
}




