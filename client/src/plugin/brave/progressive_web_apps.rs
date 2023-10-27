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

    pub fn new(plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>) -> Plugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        let pwa_entries_result = Plugin::all_entries();
        if let Err(error) = pwa_entries_result {
            log::error!(
                target: "brave-progressive-web-apps",
                "{:?}", error
            );
            panic!();
        }

        return Plugin {
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("brave-progressive-web-apps"),
                priority: 28,
                title: String::from("󰀻 Progressive Web Apps"),
                app_channel_out,
                entries: pwa_entries_result.unwrap(),
            },
        };
    }

    fn all_entries() -> anyhow::Result<Vec<crate::model::Entry>> {
        let bookmarks_root: crate::plugin::brave::utils::Bookmark =
            crate::plugin::brave::utils::read_bookmarks_file()?;

        let folder_name = String::from("Progressive Web Apps");
        let pwa_folder = bookmarks_root
            .find_bookmarks_folder_recursive(&folder_name)
            .ok_or(anyhow::anyhow!(
                "Unable to find a bookmarks folder named '{}'.",
                folder_name
            ))?;

        let pwa_bookmarks = pwa_folder
            .get_bookmarks_recursive(&vec![])
            .into_iter()
            .map(|bookmark| bookmark.into())
            .collect();

        return Ok(pwa_bookmarks);
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
        std::process::Command::new("brave")
            .arg(format!("--app={}", entry_id))
            .spawn()
            .context(format!(
                "Failed to launch brave in app mode while activating entry with id '{}'.",
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
