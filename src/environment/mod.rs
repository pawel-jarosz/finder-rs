use std::path::PathBuf;
use std::process::exit;

mod configuration_selector;

pub enum EnvironmentSetupStatus {
    UseFallback(PathBuf),
    Failed,
    Success(PathBuf)
}

mod internal {
    use crate::environment::configuration_selector::ConfigurationSelection;
    use super::*;

    fn handle_configuration(selection: ConfigurationSelection) -> EnvironmentSetupStatus {
        let (path, profiled_path) = match selection {
            ConfigurationSelection::ConfiguredSettings(path) => {
                let exists = path.exists();
                (Some(path), exists)
            }
            ConfigurationSelection::ConstructedPath(path) => {
                let exists = path.exists();
                (Some(path), exists)
            }
            ConfigurationSelection::None => {
                (None, false)
            }
        };

        if profiled_path && path.is_some() {
            EnvironmentSetupStatus::Success(path.unwrap())
        }
        else if !profiled_path && path.is_some() {
            EnvironmentSetupStatus::UseFallback(path.unwrap())
        }
        else {
            EnvironmentSetupStatus::Failed
        }
    }

    fn try_fallback(path: PathBuf) -> EnvironmentSetupStatus {
        let result = configuration_selector::dump_configuration(path);
        if let Some(fallback_path) = result {
            EnvironmentSetupStatus::UseFallback(fallback_path)
        }
        else {
            EnvironmentSetupStatus::Failed
        }

    }

    pub fn setup(env_provider: &dyn configuration_selector::EnvProvider) -> EnvironmentSetupStatus {
        let expected_configuration = configuration_selector::select_configuration(env_provider);
        let configuration_state = handle_configuration(expected_configuration);
        match configuration_state {
            EnvironmentSetupStatus::Success(path) => { EnvironmentSetupStatus::Success(path) }
            EnvironmentSetupStatus::UseFallback(path) => { try_fallback(path) }
            EnvironmentSetupStatus::Failed => EnvironmentSetupStatus::Failed
        }
    }
}

pub fn setup() -> EnvironmentSetupStatus {
    let dir_env = configuration_selector::StdEnv {};
    internal::setup(&dir_env)
}