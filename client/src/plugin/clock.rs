use anyhow::Context;
use iced::futures::StreamExt;

pub struct Plugin {
    plugin: crate::model::Plugin,
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
            last_query: String::new(),
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("clock"),
                priority: 10,
                title: String::from("󰅐 Clock"),
                app_channel_out,
                entries: Plugin::all_entries(),
            },
        };
    }

    fn all_entries() -> Vec<crate::model::Entry> {
        let date = chrono::Local::now();
        return vec![
            crate::model::Entry {
                id: String::from("time-entry"),
                title: date.format("%H:%M:%S").to_string(),
                action: String::from(""),
                meta: String::from("Clock Time"),
            },
            crate::model::Entry {
                id: String::from("date"),
                title: date.format("%A, %_d. %B %Y").to_string(),
                action: String::from(""),
                meta: String::from("Clock Date"),
            },
        ];
    }

    async fn main(&mut self) -> ! {
        let register_plugin_result = self.register_plugin();
        if let Err(error) = register_plugin_result {
            log::error!(
                target: self.plugin.id.as_str(),
                "{:?}", error
            );
            panic!();
        }

        loop {
            let update_result = self.update().await;
            if let Err(error) = update_result {
                log::warn!(
                    target: self.plugin.id.as_str(),
                    "{:?}", error
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
        let plugin_request_future = self.plugin_channel_in.select_next_some();
        let plugin_request =
            async_std::future::timeout(std::time::Duration::from_secs(1), plugin_request_future)
                .await
                .unwrap_or(crate::model::PluginRequest::Timeout);

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(query)?,
            crate::model::PluginRequest::Timeout => {
                self.plugin.entries = Plugin::all_entries();
                self.search(self.last_query.clone())?;
            }
            crate::model::PluginRequest::Activate(_) => (),
        }

        return Ok(());
    }

    fn search(&mut self, query: String) -> anyhow::Result<()> {
        let filtered_entries = crate::plugin::utils::search(self.plugin.entries.clone(), &query);

        self.plugin_channel_out
            .try_send(crate::Message::UpdateEntries(
                self.plugin.id.clone(),
                filtered_entries,
            ))
            .context(format!(
                "Failed to send message to update entries while searching for '{}'.",
                query
            ))?;

        self.last_query = query;

        return Ok(());
    }
}
