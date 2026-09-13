use std::fs;
use crate::configuration::ConfigurationHandler;
use crate::environment::EnvironmentSetupStatus;

mod environment;
mod configuration;

fn main() {
    let found_configuration = match environment::setup() {
        EnvironmentSetupStatus::Success(path) => Some(path),
        EnvironmentSetupStatus::UseFallback(path) => Some(path),
        EnvironmentSetupStatus::Failed => None
    };
    let Some(configuration_file) = found_configuration else {
        panic!("Couldn't find configuration file in expected_paths.");
    };
    let configuration_content = fs::read_to_string(&configuration_file);
    if let Err(_) = configuration_content {
        panic!("Couldn't read configuration file.");
    }
    let configuration =
        ConfigurationHandler::load_configuration(&configuration_content.unwrap());
}