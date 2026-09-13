use crate::args::{CollectionsCommand, PlacesCommand, CommandsCommand, Shell};
use crate::configuration::ConfigurationHandler;

pub fn init(command: Shell) {

}

pub fn collections(command: CollectionsCommand, configuration_handler: ConfigurationHandler) {
    match command {
        CollectionsCommand::List => {}
        CollectionsCommand::Activate { .. } => {}
    }
}

pub fn places(command: PlacesCommand) {

}

pub fn commands(command: CommandsCommand) {

}
