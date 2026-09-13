use crate::environment::configuration;

pub trait EnvProvider {
    fn var(&self, key: &str) -> Result<String, std::env::VarError>;
}

pub struct StdEnv;

impl EnvProvider for StdEnv {
    fn var(&self, key: &str) -> Result<String, std::env::VarError> {
        std::env::var(key)
    }
}

pub enum ConfigurationSelection {
    DefaultConfiguration
}

pub(crate) fn select_configuration(env_provider: &dyn EnvProvider) -> ConfigurationSelection {
    ConfigurationSelection::DefaultConfiguration
}
