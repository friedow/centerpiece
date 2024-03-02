use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct DisksPlugin {
    disks: sysinfo::Disks,
    entries: Vec<crate::model::Entry>,
}

impl Plugin for DisksPlugin {
    fn id() -> &'static str {
        "resource_monitor_disks"
    }

    fn priority() -> u32 {
        12
    }

    fn title() -> &'static str {
        "󱛟 Disks"
    }

    fn update_timeout() -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(2))
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.disks.refresh_list();
        self.entries.clear();

        for disk in &self.disks {
            let mount_point = disk
                .mount_point()
                .to_str()
                .context("Unable to convert mount point path to string.")?
                .to_string();

            let used_space = disk.total_space() - disk.available_space();
            let perentage_used = 100 * used_space / disk.total_space();
            let total_space_in_gb = disk.total_space() as f64 / 10_f64.powf(9.);
            let used_space_in_gb = used_space as f64 / 10_f64.powf(9.);

            let title = format!(
                "{} {}% ({:.2}gb / {:.2}gb)",
                &mount_point, perentage_used, used_space_in_gb, total_space_in_gb
            );

            self.entries.push(crate::model::Entry {
                id: mount_point,
                title,
                action: String::from(""),
                meta: String::from("Resource Monitor Disks"),
                command: None,
            });
        }

        Ok(())
    }

    fn new() -> Self {
        Self {
            disks: sysinfo::Disks::new(),
            entries: vec![],
        }
    }
}

impl Default for DisksPlugin {
    fn default() -> Self {
        Self::new()
    }
}
