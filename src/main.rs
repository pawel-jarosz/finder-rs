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
        panic!("Couldn't find configuration file in current working directory.");
    };

    let configuration = configuration::Configuration::new();
}