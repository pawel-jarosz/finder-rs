use std::path::PathBuf;
use std::process::exit;
use dirs::config_dir;

mod configuration;

pub enum EnvironmentSetupStatus {
    CustomConfiguration,
    Success
}

mod internal {
    use super::*;

    pub fn setup(env_provider: &dyn configuration::EnvProvider) -> EnvironmentSetupStatus {
        let configuration = configuration::select_configuration(env_provider);
        EnvironmentSetupStatus::Success
    }
}

pub fn setup() -> EnvironmentSetupStatus {
    let std_env = configuration::StdEnv {};
    internal::setup(&std_env)
}

mod tests {
    #[cfg(test)]
    fn test_priorities() {

    }
}