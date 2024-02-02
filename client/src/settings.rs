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
        let home_directory = std::env::var("HOME").map_err(|_| {
            ConfigError::Message("Could read HOME environment variable".to_string())
        })?;
        Config::builder()
            .add_source(config::File::new("config", config::FileFormat::Yaml))
            .add_source(
                config::File::new(
                    &format!("{home_directory}/.config/centerpiece/config"),
                    config::FileFormat::Yaml,
                )
                .required(false),
            )
            .build()?
            .try_deserialize()
    }
}
