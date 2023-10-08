use std::format;

use iced::futures::StreamExt;
use sysinfo::{CpuExt, DiskExt, SystemExt};

pub struct ResourceMonitorPlugin {
    sysinfo: sysinfo::System,
    cpu_plugin: crate::model::Plugin,
    disk_plugin: crate::model::Plugin,
    memory_plugin: crate::model::Plugin,
    last_query: String,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}

impl ResourceMonitorPlugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<ResourceMonitorPlugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = ResourceMonitorPlugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> ResourceMonitorPlugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        return ResourceMonitorPlugin {
            sysinfo: sysinfo::System::new_all(),
            last_query: String::new(),
            plugin_channel_in,
            plugin_channel_out,
            cpu_plugin: crate::model::Plugin {
                id: String::from("cpu-usage"),
                priority: 13,
                title: String::from("󰅐 CPU"),
                app_channel_out: app_channel_out.clone(),
                entries: vec![],
            },
            disk_plugin: crate::model::Plugin {
                id: String::from("disk-usage"),
                priority: 12,
                title: String::from("󰅐 Disks"),
                app_channel_out: app_channel_out.clone(),
                entries: vec![],
            },
            memory_plugin: crate::model::Plugin {
                id: String::from("memory-usage"),
                priority: 11,
                title: String::from("󰅐 Memory"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    async fn main(&mut self) -> ! {
        self.register_plugin();
        self.update_entries();

        loop {
            self.update().await;
        }
    }

    fn register_plugin(&mut self) {
        let register_cpu_plugin_result = self
            .plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.cpu_plugin.clone()));
        if let Err(error) = register_cpu_plugin_result {
            log::warn!(
                error = log::as_error!(error);
                "Failed to register cpu plugin",
            );
        }

        let register_disk_plugin_result = self
            .plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.disk_plugin.clone()));
        if let Err(error) = register_disk_plugin_result {
            log::warn!(
                error = log::as_error!(error);
                "Failed to register disk plugin",
            );
        }

        let register_memory_plugin_result = self
            .plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.memory_plugin.clone()));
        if let Err(error) = register_memory_plugin_result {
            log::warn!(
                error = log::as_error!(error);
                "Failed to register memory plugin",
            );
        }
    }

    async fn update(&mut self) {
        let plugin_request_future = self.plugin_channel_in.select_next_some();
        let plugin_request =
            async_std::future::timeout(std::time::Duration::from_secs(1), plugin_request_future)
                .await
                .unwrap_or(crate::model::PluginRequest::Timeout);

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(query),
            crate::model::PluginRequest::Timeout => self.update_entries(),
            crate::model::PluginRequest::Activate(_) => (),
        }
    }

    fn update_entries(&mut self) {
        self.sysinfo.refresh_all();

        self.update_cpu_entries();
        self.update_disk_entries();
        self.update_memory_entries();

        self.search(self.last_query.clone());
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

    fn update_disk_entries(&mut self) {
        self.disk_plugin.entries.clear();
        for disk in self.sysinfo.disks() {
            let mount_point_option = disk.mount_point().to_str();
            if mount_point_option.is_none() {
                log::warn!("Unable to convert mount point path to string.",);
                continue;
            }
            let mount_point = mount_point_option.unwrap().to_string();

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

    fn search(&mut self, query: String) {
        for plugin in vec![&self.cpu_plugin, &self.disk_plugin, &self.memory_plugin] {
            let filtered_entries = crate::plugin::utils::search(plugin.entries.clone(), &query);

            self.plugin_channel_out
                .try_send(crate::Message::Clear(plugin.id.clone()))
                .ok();

            for entry in filtered_entries {
                self.plugin_channel_out
                    .try_send(crate::Message::AppendEntry(plugin.id.clone(), entry))
                    .ok();
            }
        }

        self.last_query = query;
    }
}
