use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitRepositoriesSettings {
    pub commands: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PluginSettings {
    pub git_repositories: GitRepositoriesSettings,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub plugin: PluginSettings,
}

impl Default for Settings {
    fn default() -> Self {
        pub const DEFAULT_CONFIG: &[u8] =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../", "config.yml"));

        let config_result = serde_yaml::from_slice(&DEFAULT_CONFIG);
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
        if let Err(_) = config_file_result {
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
