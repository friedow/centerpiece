use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitRepositoriesSettings {
    pub commands: Vec<Vec<String>>,
}

impl Default for GitRepositoriesSettings {
    fn default() -> Self {
        Self {
            commands: vec![
                vec![
                    "alacritty".into(),
                    "--command".into(),
                    "nvim".into(),
                    "$GIT_DIRECTORY".into(),
                ],
                vec![
                    "alacritty".into(),
                    "--working-directory".into(),
                    "$GIT_DIRECTORY".into(),
                ],
            ],
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PluginSettings {
    #[serde(default)]
    pub git_repositories: GitRepositoriesSettings,
}

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub plugin: PluginSettings,
}

impl Settings {
    pub fn new() -> Self {
        let config_directory_result = crate::plugin::utils::centerpiece_config_directory();
        if let Err(error) = config_directory_result {
            log::error!(
            error = log::error!("{:?}", error);
            "Unable to find config directory.",
            );
            panic!();
        }
        let config_directory = config_directory_result.unwrap();
        let config_file_path = format!("{config_directory}/config.yml");

        let config_file_result = std::fs::File::open(config_file_path);
        if config_file_result.is_err() {
            log::info!("No custom config file found, falling back to default.");
            return Self::default();
        }
        let config_file = config_file_result.unwrap();
        let config_result = serde_yaml::from_reader(config_file);
        if let Err(error) = config_result {
            log::error!(
            error = log::error!("{:?}", error);
            "Config file does not match settings struct.",
            );
            panic!();
        }
        config_result.unwrap()
    }
}
