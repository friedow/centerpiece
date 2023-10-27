use anyhow::Context;
use iced::futures::StreamExt;

pub struct Plugin {
    plugin: crate::model::Plugin,
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
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("git-repositories"),
                priority: 28,
                title: String::from("󰘬 Git Repositories"),
                app_channel_out,
                entries: Plugin::all_entries(),
            },
        };
    }

    fn all_entries() -> Vec<crate::model::Entry> {
        let git_repository_paths: Vec<String> =
            crate::plugin::utils::read_index_file("git-repositories-index.json");

        let home = std::env::var("HOME").unwrap_or(String::from(""));

        return git_repository_paths
            .into_iter()
            .filter_map(|git_repository_path| {
                let git_repository_display_name = git_repository_path.replacen(&home, "~", 1);

                return Some(crate::model::Entry {
                    id: git_repository_path,
                    title: git_repository_display_name,
                    action: String::from("focus"),
                    meta: String::from("windows"),
                });
            })
            .collect();
    }

    async fn main(&mut self) -> ! {
        let register_plugin_result = self.register_plugin();
        if let Err(error) = register_plugin_result {
            log::error!(
                target: self.plugin.id.as_str(),
                "{:?}", error,
            );
            panic!();
        }

        loop {
            let update_result = self.update().await;
            if let Err(error) = update_result {
                log::warn!(
                    target: self.plugin.id.as_str(),
                    "{:?}", error,
                );
            }
        }
    }

    fn register_plugin(&mut self) -> anyhow::Result<()> {
        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.plugin.clone()))
            .context("Failed to send message to register plugin.")?;
        return Ok(());
    }

    async fn update(&mut self) -> anyhow::Result<()> {
        let plugin_request = self.plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(&query)?,
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => self.activate(entry_id)?,
        }

        return Ok(());
    }

    fn search(&mut self, query: &String) -> anyhow::Result<()> {
        let filtered_entries = crate::plugin::utils::search(self.plugin.entries.clone(), query);

        self.plugin_channel_out
            .try_send(crate::Message::UpdateEntries(
                self.plugin.id.clone(),
                filtered_entries,
            ))
            .context(format!(
                "Failed to send message to update entries while searching for '{}'.",
                query
            ))?;

        return Ok(());
    }

    fn activate(&mut self, entry_id: String) -> anyhow::Result<()> {
        std::process::Command::new("alacritty")
            .arg("--working-directory")
            .arg(&entry_id)
            .spawn()
            .context(format!(
                "Failed to launch terminal while activating entry with id '{}'.",
                entry_id
            ))?;

        std::process::Command::new("sublime_text")
            .arg("--new-window")
            .arg(&entry_id)
            .spawn()
            .context(format!(
                "Failed to launch editor while activating entry with id '{}'.",
                entry_id
            ))?;

        std::process::Command::new("sublime_merge")
            .arg("--new-window")
            .arg(&entry_id)
            .spawn()
            .context(format!(
                "Failed to launch git ui while activating entry with id '{}'.",
                entry_id
            ))?;

        self.plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry_id
            ))?;

        return Ok(());
    }
}
