mod args;
mod configuration;
mod cache;

use std::fs;

use args::Cli;
use configuration::Configuration;
use crate::args::{Commands, Parser};
use crate::args::CollectionsCommand::{List, Set};
use crate::cache::Cache;

fn list_collections(cache: &Cache) {
    println!("Available collections:");
    for item in &cache.configuration.collections {
        let status = if item.0 == cache.get_current_collection() {
            "current"
        } else {
            ""
        };

        println!("{:<20} | {:<10} | {}", item.0, status, item.1.description);
    }
}

fn set_current_collection(cache: &mut Cache, collection_name: String) {
    if cache.configuration.collections.contains_key(&collection_name) {
        cache.set_current_collection(collection_name);
    }
    else {
        eprintln!("Invalid collection name! Please use `collection list` command...");
    }
}

fn main() {
    let cli = Cli::parse();
    let settings_content = fs::read_to_string(cli.settings.unwrap())
        .expect("Unable to read settings file");
    let Ok(configuration) = Configuration::load_from_json(&settings_content) else {
        eprintln!("Configuration file is invalid or is not available");
        std::process::exit(1);
    };

    let mut cache = cache::load_cache(configuration);

    match cli.command {
        Commands::Collections { command } => match command {
            List => {
                list_collections(&cache);
            }
            Set{collection_name} => {
                set_current_collection(&mut cache, collection_name);
            }
        }
    }

    if cache.is_changed() {
        cache.dump()
    }
}
