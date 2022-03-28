pub mod git_repositories;
pub mod open_windows;
pub mod plugin;
pub mod system_monitor;

pub fn list() -> Vec<Box<dyn plugin::Plugin>> {
    return vec![
        Box::new(git_repositories::GitRepositoriesPlugin()),
        Box::new(open_windows::OpenWindowsPlugin()),
        Box::new(system_monitor::SystemMonitorPlugin()),
    ];
}
