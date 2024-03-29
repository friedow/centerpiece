use serde::Deserialize;

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct ApplicationsPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ApplicationsPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveBookmarksPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for BraveBookmarksPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveHistoryPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for BraveHistoryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveProgressiveWebAppsSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for BraveProgressiveWebAppsSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClockPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ClockPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct GitRepositoriesPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default = "default_commands")]
    pub commands: Vec<Vec<String>>,
}

fn default_commands() -> Vec<Vec<String>> {
    vec![
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
    ]
}

impl Default for GitRepositoriesPluginSettings {
    fn default() -> Self {
        Self {
            enable: true,
            commands: default_commands(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorBatteryPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorBatteryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorCpuPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorCpuPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorDisksSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorDisksSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorMemoryPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorMemoryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for SystemPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct WifiPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for WifiPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct SwayWindowsPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for SwayWindowsPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PluginSettings {
    #[serde(default)]
    pub applications: ApplicationsPluginSettings,
    #[serde(default)]
    pub brave_bookmarks: BraveBookmarksPluginSettings,
    #[serde(default)]
    pub brave_history: BraveHistoryPluginSettings,
    #[serde(default)]
    pub brave_progressive_web_apps: BraveProgressiveWebAppsSettings,
    #[serde(default)]
    pub clock: ClockPluginSettings,
    #[serde(default)]
    pub git_repositories: GitRepositoriesPluginSettings,
    #[serde(default)]
    pub resource_monitor_battery: ResourceMonitorBatteryPluginSettings,
    #[serde(default)]
    pub resource_monitor_cpu: ResourceMonitorCpuPluginSettings,
    #[serde(default)]
    pub resource_monitor_disks: ResourceMonitorDisksSettings,
    #[serde(default)]
    pub resource_monitor_memory: ResourceMonitorMemoryPluginSettings,
    #[serde(default)]
    pub sway_windows: SwayWindowsPluginSettings,
    #[serde(default)]
    pub system: SystemPluginSettings,
    #[serde(default)]
    pub wifi: WifiPluginSettings,
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

impl std::convert::TryFrom<crate::cli::CliArgs> for Settings {
    type Error = anyhow::Error;

    fn try_from(args: crate::cli::CliArgs) -> Result<Self, Self::Error> {
        let maybe_config_file_path = args.config;
        let config_file_path = maybe_config_file_path.unwrap_or_else(|| {
            crate::plugin::utils::centerpiece_default_config_path().unwrap_or_else(|error| {
                log::error!(
                    error = log::error!("{:?}", error);
                    "Unable to find default config file.",
                );
                panic!();
            })
        });
        let config_file_result = std::fs::File::open(config_file_path);
        if config_file_result.is_err() {
            log::info!("No custom config file found, falling back to default.");
            return Ok(Self::default());
        }
        let config_file = config_file_result?;
        let config_result = serde_yaml::from_reader(config_file);
        if let Err(ref error) = config_result {
            log::error!(
            error = log::error!("{:?}", error);
            "Config file does not match settings struct.",
            );
        }
        Ok(config_result?)
    }
}
