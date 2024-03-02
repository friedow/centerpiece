use crate::plugin::utils::Plugin;

pub struct CpuPlugin {
    sysinfo: sysinfo::System,
    entries: Vec<crate::model::Entry>,
}

impl Plugin for CpuPlugin {
    fn id() -> &'static str {
        "resource_monitor_cpu"
    }

    fn priority() -> u32 {
        13
    }

    fn title() -> &'static str {
        "󰍛 CPU"
    }

    fn update_timeout() -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(2))
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.sysinfo.refresh_cpu();

        self.entries.clear();
        for cpu_core in self.sysinfo.cpus() {
            self.entries.push(crate::model::Entry {
                id: cpu_core.name().to_string(),
                title: format!(
                    "{}: {}% {}MHz",
                    cpu_core.name(),
                    cpu_core.cpu_usage() as i32,
                    cpu_core.frequency()
                ),
                action: String::from(""),
                meta: String::from("Resource Monitor CPU"),
                command: None,
            });
        }

        Ok(())
    }

    fn new() -> Self {
        Self {
            sysinfo: sysinfo::System::new_all(),
            entries: vec![],
        }
    }
}

impl Default for CpuPlugin {
    fn default() -> Self {
        Self::new()
    }
}
