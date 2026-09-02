mod args;
mod configuration;
mod cache;
mod bookmark_collection;

use std::fs;
use args::Cli;
use configuration::Configuration;

use crate::args::{Commands, CommandsCommand, Parser, PlacesCommand};
use crate::args::CollectionsCommand::{List, Set};
use crate::bookmark_collection::BookmarkCollection;
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

fn get_place(cache: &Cache, place: String) {
    let current_collection = cache.get_current_collection();

    let Some(collection_detail) = cache.configuration.collections.get(current_collection) else {
        return;
    };
    let content = fs::read_to_string(&collection_detail.path).expect("File not found!");
    let bookmarks = BookmarkCollection::load_from_json(content.as_str());
    if bookmarks.places.contains_key(&place) {
        print!("{}", bookmarks.places.get(&place).unwrap().path);
    }
}

fn get_command(cache: &Cache, command: String) {
    let current_collection = cache.get_current_collection();

    let Some(collection_detail) = cache.configuration.collections.get(current_collection) else {
        return;
    };
    let content = fs::read_to_string(&collection_detail.path).expect("File not found!");
    let bookmarks = BookmarkCollection::load_from_json(content.as_str());
    if bookmarks.commands.contains_key(&command) {
        print!("{}", bookmarks.commands.get(&command).unwrap().command);
    }
}

fn main() {
    let cli = Cli::parse();
    let settings_content = fs::read_to_string(shellexpand::full(&cli.settings)
        .expect("Cannot resolve environmental variable").as_ref())
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
        },
        Commands::Places { command } => match command {
            PlacesCommand::Get { place } => get_place(&cache, place),
        },
        Commands::Commands { command } => match command {
            CommandsCommand::Get { command } => get_command(&cache, command),
        }
    }


    if cache.is_changed() {
        cache.dump()
    }
}
