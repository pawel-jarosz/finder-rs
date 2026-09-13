use std::path::PathBuf;

#[cfg_attr(test, mockall::automock)]
pub trait EnvProvider {
    fn var(&self, key: &str) -> Result<String, std::env::VarError>;

    fn config_dir(&self) -> Option<PathBuf>;

    fn home_dir(&self) -> Option<PathBuf>;
}

pub struct StdEnv;

impl EnvProvider for StdEnv {
    fn var(&self, key: &str) -> Result<String, std::env::VarError> {
        std::env::var(key)
    }

    fn config_dir(&self) -> Option<PathBuf> {
        dirs::config_dir()
    }

    fn home_dir(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }
}

pub enum ConfigurationSelection {
    ConfiguredSettings(PathBuf),
    ConstructedPath(PathBuf),
    None
}

fn create_configuration_path(path: PathBuf, postfix: &str) -> PathBuf {
    let split = postfix.split("/");
    let mut result = path;
    for part in split {
        result = result.join(part);
    }
    result
}

pub(crate) fn select_configuration(env_provider: &dyn EnvProvider) -> ConfigurationSelection {
    if let Ok(path) = env_provider.var("FINDER_RS_CONFIG") {
        return ConfigurationSelection::ConfiguredSettings(path.into());
    }
    if let Some(path) = env_provider.config_dir() {
        return ConfigurationSelection::ConstructedPath(create_configuration_path(path, "finder-rs/settings.yaml").into());
    }
    if let Some(path) = env_provider.home_dir() {
        return ConfigurationSelection::ConstructedPath(create_configuration_path(path, ".finder-rs/settings.yaml").into());
    }
    ConfigurationSelection::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::VarError;
    use std::ffi::OsString;
    use mockall::predicate::eq;

    #[test]
    fn select_configuration_prefers_explicit_path() {
        let expected = PathBuf::from("custom").join("settings.yaml");
        let mut env = MockEnvProvider::new();
        let configured_path = expected.to_str().unwrap().to_owned();
        env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(move |_| Ok(configured_path));
        env.expect_config_dir().never();
        env.expect_home_dir().never();

        assert!(matches!(
            select_configuration(&env),
            ConfigurationSelection::ConfiguredSettings(path) if path == expected
        ));
    }

    #[test]
    fn select_configuration_preserves_empty_explicit_path() {
        let mut env = MockEnvProvider::new();
        env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(|_| Ok(String::new()));
        env.expect_config_dir().never();
        env.expect_home_dir().never();

        assert!(matches!(
            select_configuration(&env),
            ConfigurationSelection::ConfiguredSettings(path) if path == PathBuf::new()
        ));
    }

    #[test]
    fn select_configuration_falls_back_to_config_directory_on_env_errors() {
        for error in [VarError::NotPresent, VarError::NotUnicode(OsString::from("invalid"))] {
            let mut env = MockEnvProvider::new();
            env.expect_var()
                .with(eq("FINDER_RS_CONFIG"))
                .times(1)
                .return_once(move |_| Err(error));
            env.expect_config_dir()
                .times(1)
                .return_once(|| Some(PathBuf::from("config")));
            env.expect_home_dir().never();

            assert!(matches!(
                select_configuration(&env),
                ConfigurationSelection::ConstructedPath(path)
                    if path == PathBuf::from("config").join("finder-rs").join("settings.yaml")
            ));
        }
    }

    #[test]
    fn select_configuration_falls_back_to_home_directory() {
        let mut env = MockEnvProvider::new();
        env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(|_| Err(VarError::NotPresent));
        env.expect_config_dir().times(1).return_once(|| None);
        env.expect_home_dir()
            .times(1)
            .return_once(|| Some(PathBuf::from("home")));

        assert!(matches!(
            select_configuration(&env),
            ConfigurationSelection::ConstructedPath(path)
                if path == PathBuf::from("home").join(".finder-rs").join("settings.yaml")
        ));
    }

    #[test]
    fn select_configuration_returns_none_without_any_location() {
        let mut env = MockEnvProvider::new();
        env.expect_var()
            .with(eq("FINDER_RS_CONFIG"))
            .times(1)
            .return_once(|_| Err(VarError::NotPresent));
        env.expect_config_dir().times(1).return_once(|| None);
        env.expect_home_dir().times(1).return_once(|| None);

        assert!(matches!(select_configuration(&env), ConfigurationSelection::None));
    }

    fn assert_paths(cases: &[(&str, &str, &str)]) {
        for &(base, postfix, expected) in cases {
            let result = create_configuration_path(PathBuf::from(base), postfix);
            assert_eq!(
                result,
                PathBuf::from(expected),
                "base: {base:?}, postfix: {postfix:?}"
            );
        }
    }
    #[test]
    #[cfg(windows)]
    fn windows_configuration_paths() {
        assert_paths(&[
            (
                r"C:\Users\user\AppData\Roaming",
                "finder-rs/settings.yaml",
                r"C:\Users\user\AppData\Roaming\finder-rs\settings.yaml",
            ),
            (
                r"C:\Users\user",
                ".finder-rs/settings.yaml",
                r"C:\Users\user\.finder-rs\settings.yaml",
            ),
            (
                r"D:\Custom Config\",
                "finder-rs/settings.yaml",
                r"D:\Custom Config\finder-rs\settings.yaml",
            ),
            (
                r"C:\Users\Paweł Żółć",
                ".finder-rs/settings.yaml",
                r"C:\Users\Paweł Żółć\.finder-rs\settings.yaml",
            ),
            (
                r"\\server\share\config",
                "finder-rs/settings.yaml",
                r"\\server\share\config\finder-rs\settings.yaml",
            ),
            (
                r"\\?\C:\Users\user\AppData\Roaming",
                "finder-rs/settings.yaml",
                r"\\?\C:\Users\user\AppData\Roaming\finder-rs\settings.yaml",
            ),
            (
                r"C:\",
                "finder-rs/settings.yaml",
                r"C:\finder-rs\settings.yaml",
            ),
        ]);
    }

    #[test]
    #[cfg(unix)]
    fn macos_configuration_paths() {
        assert_paths(&[
            (
                "/Users/user/Library/Application Support",
                "finder-rs/settings.yaml",
                "/Users/user/Library/Application Support/finder-rs/settings.yaml",
            ),
            (
                "/Users/user",
                ".finder-rs/settings.yaml",
                "/Users/user/.finder-rs/settings.yaml",
            ),
        ]);
    }

    #[test]
    #[cfg(unix)]
    fn linux_configuration_paths() {
        assert_paths(&[
            (
                "/home/user/.config",
                "finder-rs/settings.yaml",
                "/home/user/.config/finder-rs/settings.yaml",
            ),
            (
                "/home/user",
                ".finder-rs/settings.yaml",
                "/home/user/.finder-rs/settings.yaml",
            ),
            (
                "/custom/config",
                "finder-rs/settings.yaml",
                "/custom/config/finder-rs/settings.yaml",
            ),
        ]);
    }

    #[test]
    #[cfg(unix)]
    fn freebsd_configuration_paths() {
        assert_paths(&[
            (
                "/usr/home/user/.config",
                "finder-rs/settings.yaml",
                "/usr/home/user/.config/finder-rs/settings.yaml",
            ),
            (
                "/usr/home/user",
                ".finder-rs/settings.yaml",
                "/usr/home/user/.finder-rs/settings.yaml",
            ),
        ]);
    }

    #[test]
    #[cfg(unix)]
    fn unix_configuration_path_edge_cases() {
        assert_paths(&[
            (
                "/home/user/.config/",
                "finder-rs/settings.yaml",
                "/home/user/.config/finder-rs/settings.yaml",
            ),
            (
                "/home/Paweł Żółć",
                ".finder-rs/settings.yaml",
                "/home/Paweł Żółć/.finder-rs/settings.yaml",
            ),
            ("/", "finder-rs/settings.yaml", "/finder-rs/settings.yaml"),
        ]);
    }
}
