use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApplicationsPluginSettings {
    pub enable: bool,
}

impl Default for ApplicationsPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveBookmarksPluginSettings {
    pub enable: bool,
}

impl Default for BraveBookmarksPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveHistoryPluginSettings {
    pub enable: bool,
}

impl Default for BraveHistoryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveProgressiveWebAppsSettings {
    pub enable: bool,
}

impl Default for BraveProgressiveWebAppsSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClockPluginSettings {
    pub enable: bool,
}

impl Default for ClockPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct GitRepositoriesPluginSettings {
    pub enable: bool,
    pub commands: Vec<Vec<String>>,
}

impl Default for GitRepositoriesPluginSettings {
    fn default() -> Self {
        Self {
            enable: true,
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

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorBatteryPluginSettings {
    pub enable: bool,
}

impl Default for ResourceMonitorBatteryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorCpuPluginSettings {
    pub enable: bool,
}

impl Default for ResourceMonitorCpuPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorDisksSettings {
    pub enable: bool,
}

impl Default for ResourceMonitorDisksSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorMemoryPluginSettings {
    pub enable: bool,
}

impl Default for ResourceMonitorMemoryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemPluginSettings {
    pub enable: bool,
}

impl Default for SystemPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct WifiPluginSettings {
    pub enable: bool,
}

impl Default for WifiPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct WindowsPluginSettings {
    pub enable: bool,
}

impl Default for WindowsPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PluginSettings {
    pub applications: ApplicationsPluginSettings,
    pub brave_bookmarks: BraveBookmarksPluginSettings,
    pub brave_history: BraveHistoryPluginSettings,
    pub brave_progressive_web_apps: BraveProgressiveWebAppsSettings,
    pub clock: ClockPluginSettings,
    pub git_repositories: GitRepositoriesPluginSettings,
    pub resource_monitor_battery: ResourceMonitorBatteryPluginSettings,
    pub resource_monitor_cpu: ResourceMonitorCpuPluginSettings,
    pub resource_monitor_disks: ResourceMonitorDisksSettings,
    pub resource_monitor_memory: ResourceMonitorMemoryPluginSettings,
    pub system: SystemPluginSettings,
    pub wifi: WifiPluginSettings,
    pub windows: WindowsPluginSettings,
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
