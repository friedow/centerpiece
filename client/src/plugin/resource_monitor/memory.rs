use crate::plugin::utils::Plugin;
use sysinfo::SystemExt;

pub struct MemoryPlugin {
    sysinfo: sysinfo::System,
    entries: Vec<crate::model::Entry>,
}

impl Plugin for MemoryPlugin {
    fn id() -> &'static str {
        return "memory";
    }

    fn priority() -> u32 {
        return 11;
    }

    fn title() -> &'static str {
        return "󱓱 Memory";
    }

    fn update_timeout() -> Option<std::time::Duration> {
        return Some(std::time::Duration::from_secs(2));
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
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
        });

        return Ok(());
    }

    fn new() -> Self {
        return Self {
            sysinfo: sysinfo::System::new_all(),
            entries: vec![],
        };
    }
}
