use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Place {
    pub path: String
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub command: String
}

#[derive(Debug, Deserialize)]
pub struct BookmarkCollection {
    pub places: HashMap<String, Place>,
    pub commands: HashMap<String, Command>,
}

impl BookmarkCollection {
    pub fn load_from_json(content: &str) -> BookmarkCollection {
        serde_json::from_str::<BookmarkCollection>(content).expect("Unable to parse cache file")
    }
}