use config::{Config, ConfigError};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GitRepositoriesSettings {
    pub editor_command: Vec<String>,
    pub git_ui_command: Vec<String>,
    pub terminal_command: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PluginSettings {
    pub git_repositories: GitRepositoriesSettings,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub plugin: PluginSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_directory = crate::plugin::utils::config_directory().map_err(|_| {
            config::ConfigError::Message("Unable to find config directory.".to_string())
        })?;
        let config_file = format!("{config_directory}/config");

        Config::builder()
            .add_source(config::File::new("config", config::FileFormat::Yaml))
            .add_source(config::File::new(&config_file, config::FileFormat::Yaml).required(false))
            .build()?
            .try_deserialize()
    }
}
