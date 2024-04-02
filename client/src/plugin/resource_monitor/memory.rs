use crate::plugin::utils::Plugin;

pub struct MemoryPlugin {
    sysinfo: sysinfo::System,
    entries: Vec<crate::model::Entry>,
}

impl Plugin for MemoryPlugin {
    fn id() -> &'static str {
        "resource_monitor_memory"
    }

    fn priority() -> u32 {
        11
    }

    fn title() -> &'static str {
        "󱓱 Memory"
    }

    fn update_timeout() -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(2))
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.sysinfo.refresh_memory();
        self.entries.clear();

        let perentage_used = 100 * self.sysinfo.used_memory() / self.sysinfo.total_memory();
        let total_memory_in_gb = self.sysinfo.total_memory() as f64 / 10_f64.powf(9.);
        let used_memory_in_gb = self.sysinfo.used_memory() as f64 / 10_f64.powf(9.);

        let title = format!(
            "{}% ({:.2}gb / {:.2}gb)",
            perentage_used, used_memory_in_gb, total_memory_in_gb
        );

        self.entries.push(crate::model::Entry {
            id: String::from("memory"),
            title,
            action: String::from(""),
            meta: String::from("Resource Monitor Memory RAM"),
            command: None,
        });

        Ok(())
    }

    fn new() -> Self {
        Self {
            sysinfo: sysinfo::System::new_all(),
            entries: vec![],
        }
    }
}

impl Default for MemoryPlugin {
    fn default() -> Self {
        Self::new()
    }
}
