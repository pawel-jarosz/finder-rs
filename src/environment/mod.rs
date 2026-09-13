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

    #[cfg_attr(test, mockall::automock)]
    pub trait FileChecker {
        fn exists(&self, path: &PathBuf) -> bool;
    }

    pub struct StdFileChecker;

    impl FileChecker for StdFileChecker {
        fn exists(&self, path: &PathBuf) -> bool {
            path.exists()
        }
    }

    fn handle_configuration(selection: ConfigurationSelection,
                            file_checker: &dyn FileChecker) -> EnvironmentSetupStatus {
        let (path, profiled_path) = match selection {
            ConfigurationSelection::ConfiguredSettings(path) => {
                let exists = file_checker.exists(&path);
                (Some(path), exists)
            }
            ConfigurationSelection::ConstructedPath(path) => {
                let exists = file_checker.exists(&path);
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

    pub fn setup(env_provider: &dyn configuration_selector::EnvProvider,
                 file_checker: &dyn FileChecker) -> EnvironmentSetupStatus {
        let expected_configuration = configuration_selector::select_configuration(env_provider);
        let configuration_state = handle_configuration(expected_configuration, file_checker);
        match configuration_state {
            EnvironmentSetupStatus::Success(path) => { EnvironmentSetupStatus::Success(path) }
            EnvironmentSetupStatus::UseFallback(path) => { try_fallback(path) }
            EnvironmentSetupStatus::Failed => EnvironmentSetupStatus::Failed
        }
    }
}

pub fn setup() -> EnvironmentSetupStatus {
    let dir_env = configuration_selector::StdEnv {};
    let file_checker = internal::StdFileChecker {};
    internal::setup(&dir_env, &file_checker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use configuration_selector::MockEnvProvider;
    use internal::MockFileChecker;
    use mockall::predicate::eq;
    use std::env::VarError;

    fn configuration_locations() -> Vec<(MockEnvProvider, PathBuf)> {
        let explicit_path = PathBuf::from("custom").join("settings.yaml");
        let configured_path = explicit_path.to_str().unwrap().to_owned();
        let mut explicit_env = MockEnvProvider::new();
        explicit_env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(move |_| Ok(configured_path));
        explicit_env.expect_config_dir().never();
        explicit_env.expect_home_dir().never();

        let mut config_env = MockEnvProvider::new();
        config_env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(|_| Err(VarError::NotPresent));
        config_env.expect_config_dir()
            .times(1)
            .return_once(|| Some(PathBuf::from("config")));
        config_env.expect_home_dir().never();

        let mut home_env = MockEnvProvider::new();
        home_env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(|_| Err(VarError::NotPresent));
        home_env.expect_config_dir().times(1).return_once(|| None);
        home_env.expect_home_dir()
            .times(1)
            .return_once(|| Some(PathBuf::from("home")));

        vec![
            (explicit_env, explicit_path),
            (config_env, PathBuf::from("config").join("finder-rs").join("settings.yaml")),
            (home_env, PathBuf::from("home").join(".finder-rs").join("settings.yaml")),
        ]
    }

    #[test]
    fn setup_succeeds_for_existing_configuration_paths() {
        for (env, expected_path) in configuration_locations() {
            let mut file_checker = MockFileChecker::new();
            file_checker.expect_exists()
                .with(eq(expected_path.clone()))
                .times(1)
                .return_const(true);

            assert!(matches!(
                internal::setup(&env, &file_checker),
                EnvironmentSetupStatus::Success(path) if path == expected_path
            ), "expected success for {expected_path:?}");
        }
    }

    #[test]
    fn setup_fails_when_selected_file_is_missing_and_fallback_is_unavailable() {
        // dump_configuration currently returns None, so a missing file leads to Failed.
        for (env, expected_path) in configuration_locations() {
            let mut file_checker = MockFileChecker::new();
            file_checker.expect_exists()
                .with(eq(expected_path.clone()))
                .times(1)
                .return_const(false);

            assert!(matches!(
                internal::setup(&env, &file_checker),
                EnvironmentSetupStatus::Failed
            ), "expected failure for missing {expected_path:?}");
        }
    }

    #[test]
    fn setup_fails_without_configuration_location_and_does_not_check_files() {
        let mut env = MockEnvProvider::new();
        env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(|_| Err(VarError::NotPresent));
        env.expect_config_dir().times(1).return_once(|| None);
        env.expect_home_dir().times(1).return_once(|| None);
        let mut file_checker = MockFileChecker::new();
        file_checker.expect_exists().never();

        assert!(matches!(
            internal::setup(&env, &file_checker),
            EnvironmentSetupStatus::Failed
        ));
    }
}
