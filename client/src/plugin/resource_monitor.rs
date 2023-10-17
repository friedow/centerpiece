use std::ops::Rem;
use anyhow::Context;
use iced::futures::StreamExt;
use sysinfo::{CpuExt, DiskExt, SystemExt};

pub struct Plugin {
    sysinfo: sysinfo::System,
    battery_plugin: crate::model::Plugin,
    cpu_plugin: crate::model::Plugin,
    disk_plugin: crate::model::Plugin,
    memory_plugin: crate::model::Plugin,
    last_query: String,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}

impl Plugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<Plugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = Plugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> Plugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        return Plugin {
            sysinfo: sysinfo::System::new_all(),
            last_query: String::new(),
            plugin_channel_in,
            plugin_channel_out,
            battery_plugin: crate::model::Plugin {
                id: String::from("battery"),
                priority: 14,
                title: String::from("󰁼 Battery"),
                app_channel_out: app_channel_out.clone(),
                entries: vec![],
            },
            cpu_plugin: crate::model::Plugin {
                id: String::from("cpu-usage"),
                priority: 13,
                title: String::from("󰍛 CPU"),
                app_channel_out: app_channel_out.clone(),
                entries: vec![],
            },
            disk_plugin: crate::model::Plugin {
                id: String::from("disk-usage"),
                priority: 12,
                title: String::from("󱛟 Disks"),
                app_channel_out: app_channel_out.clone(),
                entries: vec![],
            },
            memory_plugin: crate::model::Plugin {
                id: String::from("memory-usage"),
                priority: 11,
                title: String::from("󱓱 Memory"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    async fn main(&mut self) -> ! {
        let register_plugin_result = self.register_plugins();
        if let Err(error) = register_plugin_result {
            log::error!(
                target: "resource-monitor",
                "{:?}", error,
            );
            std::process::exit(1);
        }

        let update_entries_result = self.update_entries();
        if let Err(error) = update_entries_result {
            log::warn!(
                target: "resource-monitor",
                "{:?}", error,
            );
        }

        loop {
            let update_result = self.update().await;
            if let Err(error) = update_result {
                log::warn!(
                    target: "resource-monitor",
                    "{:?}", error,
                );
            }
        }
    }

    fn register_plugins(&mut self) -> anyhow::Result<()> {
        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.battery_plugin.clone()))
            .context("Failed to send message to register the battery plugin.")?;

        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.cpu_plugin.clone()))
            .context("Failed to send message to register the cpu plugin.")?;

        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.disk_plugin.clone()))
            .context("Failed to send message to register the disk plugin.")?;

        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.memory_plugin.clone()))
            .context("Failed to send message to register the memory plugin.")?;

        return Ok(());
    }

    async fn update(&mut self) -> anyhow::Result<()> {
        let plugin_request_future = self.plugin_channel_in.select_next_some();
        let plugin_request =
            async_std::future::timeout(std::time::Duration::from_secs(2), plugin_request_future)
                .await
                .unwrap_or(crate::model::PluginRequest::Timeout);

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(query)?,
            crate::model::PluginRequest::Timeout => self.update_entries()?,
            crate::model::PluginRequest::Activate(_) => (),
        }

        return Ok(());
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.sysinfo.refresh_all();

        self.update_battery_entries()?;
        self.update_cpu_entries();
        self.update_disk_entries()?;
        self.update_memory_entries();

        self.search(self.last_query.clone())?;
        return Ok(());
    }

    fn update_battery_entries(&mut self) -> anyhow::Result<()> {
        self.battery_plugin.entries.clear();

        let batteries = battery::Manager::new()
            .context("Failed to create battery manager.")?
            .batteries()
            .context("Failed to list batteries using the battery manager.")?;

        for battery_result in batteries {
            let battery =
                battery_result.context("Failed to get battery using the batteries iterator.")?;

            let state_of_charge = battery.state_of_charge() * 100.0;

            let time_to_full_remaining = match battery.time_to_full() {
                Some(time_to_full) => to_display(time_to_full),
                None => String::new(),
            };

            let time_to_empty_remaining = match battery.time_to_empty() {
                Some(time_to_empty) => to_display(time_to_empty),
                None => String::new(),
            };

            let title = format!(
                "{state_of_charge:.0?}% – {state}{time_to_full_remaining}{time_to_empty_remaining}",
                state = battery.state(),
            );

            self.battery_plugin.entries.push(crate::model::Entry {
                id: String::from("battery"),
                title,
                action: String::from(""),
                meta: String::from("Resource Monitor Battery"),
            });
        }

        return Ok(());
    }

    fn update_memory_entries(&mut self) {
        self.memory_plugin.entries.clear();

        let perentage_used = 100 * self.sysinfo.used_memory() / self.sysinfo.total_memory();
        let total_memory_in_gb = self.sysinfo.total_memory() as f64 / 10_f64.powf(9.);
        let used_memory_in_gb = self.sysinfo.used_memory() as f64 / 10_f64.powf(9.);

        let title = format!(
            "{}% ({:.2}gb / {:.2}gb)",
            perentage_used, used_memory_in_gb, total_memory_in_gb
        );

        self.memory_plugin.entries.push(crate::model::Entry {
            id: String::from("memory"),
            title,
            action: String::from(""),
            meta: String::from("Resource Monitor Memory RAM"),
        });
    }

    fn update_disk_entries(&mut self) -> anyhow::Result<()> {
        self.disk_plugin.entries.clear();

        for disk in self.sysinfo.disks() {
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

            self.disk_plugin.entries.push(crate::model::Entry {
                id: mount_point,
                title,
                action: String::from(""),
                meta: String::from("Resource Monitor Disks"),
            });
        }

        return Ok(());
    }

    fn update_cpu_entries(&mut self) {
        self.cpu_plugin.entries.clear();

        for cpu_core in self.sysinfo.cpus() {
            self.cpu_plugin.entries.push(crate::model::Entry {
                id: cpu_core.name().to_string(),
                title: format!(
                    "{}: {}% {}MHz",
                    cpu_core.name(),
                    cpu_core.cpu_usage() as i32,
                    cpu_core.frequency()
                ),
                action: String::from(""),
                meta: String::from("Resource Monitor Disks"),
            });
        }
    }

    fn search(&mut self, query: String) -> anyhow::Result<()> {
        for plugin in vec![
            &self.battery_plugin,
            &self.cpu_plugin,
            &self.disk_plugin,
            &self.memory_plugin,
        ] {
            let filtered_entries = crate::plugin::utils::search(plugin.entries.clone(), &query);

            self.plugin_channel_out
            .try_send(crate::Message::UpdateEntries(
                plugin.id.clone(),
                filtered_entries,
            ))
            .context(format!(
                "Failed to send message to update entries for plugin with id '{}' while searching for '{}'.",
                plugin.id,
                query
            ))?;
        }

        self.last_query = query;
        return Ok(());
    }
}

fn to_display(time_to_empty: battery::units::Time) -> String {
    let mut formatted_time_remaining = String::from(":");
    let hours = (time_to_empty.value / 60.0 / 60.0).round();
    if hours > 0.0 {
        formatted_time_remaining.push_str(format!(" {hours:.0}h").as_str())
    }
    let minutes = (time_to_empty.value / 60.0).rem(60.0).round();
    if minutes > 0.0 {
        formatted_time_remaining.push_str(format!(" {minutes:.0}m").as_str())
    }
    formatted_time_remaining.push_str(" remaining");
    formatted_time_remaining
}
