use std::fs;
use std::path::{Path, PathBuf};
use crate::args::{CollectionsCommand, PlacesCommand, CommandsCommand, Shell};
use crate::configuration::ConfigurationHandler;

pub fn init(_command: Shell) {

}

pub fn collections(command: CollectionsCommand, configuration_handler: &mut ConfigurationHandler) {
    match command {
        CollectionsCommand::List => {
            configuration_handler.list_collections();
        }
        CollectionsCommand::Activate { collection_name } => {
            configuration_handler.activate_collection(collection_name);
        }
    }
}

fn expand_tilde(path: &Path) -> PathBuf {
    if let Ok(rest) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }

    path.to_path_buf()
}

pub fn places(command: PlacesCommand, configuration: &ConfigurationHandler) {
    let collection_name = configuration.current_collection();
    let collection_info = configuration.get_collection(collection_name);
    let Some(collection_info) = collection_info else {
        eprintln!("Collection not found.");
        return;
    };
    let extended_path = expand_tilde(&collection_info.path);
    let content = fs::read_to_string(extended_path).unwrap();
    let collection = crate::collections::Collection::load(&content);
    let Ok(collection) = collection else {
        eprintln!("Failed to load collection.");
        return;
    };

    match command {
        PlacesCommand::List => { collection.list_places(); }
        PlacesCommand::Get { place_name } => { collection.get_place(&place_name); }
    };
}

pub fn commands(command: CommandsCommand, configuration: &ConfigurationHandler) {
    let collection_name = configuration.current_collection();
    let collection_info = configuration.get_collection(collection_name);
    let Some(collection_info) = collection_info else {
        eprintln!("Collection not found.");
        return;
    };
    let extended_path = expand_tilde(&collection_info.path);
    let content = fs::read_to_string(extended_path).unwrap();
    let collection = crate::collections::Collection::load(&content);
    let Ok(collection) = collection else {
        eprintln!("Failed to load collection.");
        return;
    };

    match command {
        CommandsCommand::List => { collection.list_commands(); }
        CommandsCommand::Get { command_name } => { collection.get_command(&command_name); }
    };
}
