use std::fs;
use crate::configuration::ConfigurationHandler;
use crate::environment::EnvironmentSetupStatus;
use crate::args::{Parser};

mod environment;
mod configuration;
mod args;
mod subprograms;
mod collections;

fn main() {
    let cli = args::Cli::parse();

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
        ConfigurationHandler::load_configuration(&configuration_content.unwrap(), configuration_file);
    let Ok(mut configuration) = configuration else {
        panic!("Couldn't parse configuration file.");
    };

    match cli.command {
        args::Commands::Init {
            shell
        } => subprograms::init(shell),
        args::Commands::Collections{command} => subprograms::collections(command, &mut configuration),
        args::Commands::Places {command} => subprograms::places(command, &configuration),
        args::Commands::Commands{command} => subprograms::commands(command, &configuration),
    }
    if configuration.changed {
        configuration.save();
    }
}