use std::fs;
use std::path::{Path};

use crate::configuration::Configuration;

mod internal {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Cache {
        pub current_collection: String,
    }

    impl Cache {
        pub fn new(current_collection: &str) -> Cache {
            Cache {
                current_collection: current_collection.to_string(),
            }
        }

        pub fn load_cache_from_json(content: &str) -> Cache {
            serde_json::from_str::<Cache>(content).expect("Unable to parse cache file")
        }
    }
}

pub struct Cache {
    pub configuration: Configuration,
    current_collection: String,
    changed: bool
}

impl Cache {
    pub fn new(configuration: Configuration, current_collection: String, changed: bool) -> Cache {
        Cache {
            configuration,
            current_collection,
            changed,
        }
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn dump(&self) {
        let internal_cache = internal::Cache::new(self.current_collection.as_str());
        let json = serde_json::to_string(&internal_cache).unwrap();

        fs::write(self.configuration.cache_file.as_str(), json.as_str()).unwrap();
    }

    pub fn set_current_collection(&mut self, collection_name: String) {
        self.current_collection = collection_name;
        self.changed = true;
    }

    pub fn get_current_collection(&self) -> &str {
        &self.current_collection
    }
}

pub fn load_cache(configuration: Configuration) -> Cache {
    let path = shellexpand::full(&configuration.cache_file);
    let expanded_path = path.expect("Invalid shell expand").as_ref().to_string();
    let path = Path::new(&expanded_path);
    if !path.exists() {
        let current_collection = configuration.default_collection.clone();
        println!("No such file or directory: {}", path.display());
        Cache::new(configuration, current_collection, true)
    }
    else {
        let content = fs::read_to_string(&path).expect("Unable to read file");
        let internal_cache = internal::Cache::load_cache_from_json(&content);
        Cache::new(configuration, internal_cache.current_collection, false)
    }
}
