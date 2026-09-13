use crate::args::{CollectionsCommand, PlacesCommand, CommandsCommand, Shell};
use crate::args::Commands::Places;
use crate::configuration::ConfigurationHandler;

pub fn init(command: Shell) {

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

pub fn places(command: PlacesCommand, configuration: &ConfigurationHandler) {
    match command {
        PlacesCommand::List => {}
        PlacesCommand::Get { place_name } => {}
    };
}

pub fn commands(command: CommandsCommand, configuration: &ConfigurationHandler) {

}
