use iced::futures::StreamExt;

pub struct ClockPlugin {
    plugin: crate::model::Plugin,
    last_query: String,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}

impl ClockPlugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<ClockPlugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = ClockPlugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> ClockPlugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        return ClockPlugin {
            last_query: String::new(),
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("clock"),
                priority: 10,
                title: String::from("󰅐 Clock"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    async fn main(&mut self) -> ! {
        let register_plugin_result = self.register_plugin();
        if let Err(error) = register_plugin_result {
            log::error!(
                target: self.plugin.id.as_str(),
                error = log::as_error!(error);
                "Failed to register plugin.",
            );
            std::process::exit(1);
        }

        let update_entries_result = self.update_entries();
        if let Err(error) = update_entries_result {
            log::warn!(
                target: self.plugin.id.as_str(),
                error = log::as_error!(error);
                "Failed to update entries.",
            );
        }

        loop {
            let update_result = self.update().await;
            if let Err(error) = update_result {
                log::warn!(
                    target: self.plugin.id.as_str(),
                    error = log::as_error!(error);
                    "Error during update loop.",
                );
            }
        }
    }

    fn register_plugin(
        &mut self,
    ) -> Result<(), iced::futures::channel::mpsc::TrySendError<crate::Message>> {
        return self
            .plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.plugin.clone()));
    }

    async fn update(
        &mut self,
    ) -> Result<(), iced::futures::channel::mpsc::TrySendError<crate::Message>> {
        let plugin_request_future = self.plugin_channel_in.select_next_some();
        let plugin_request =
            async_std::future::timeout(std::time::Duration::from_secs(1), plugin_request_future)
                .await
                .unwrap_or(crate::model::PluginRequest::Timeout);

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(query),
            crate::model::PluginRequest::Timeout => self.update_entries(),
            crate::model::PluginRequest::Activate(_) => Ok(()),
        }
    }

    fn update_entries(
        &mut self,
    ) -> Result<(), iced::futures::channel::mpsc::TrySendError<crate::Message>> {
        self.plugin.entries.clear();
        let date = chrono::Local::now();

        let formatted_time = date.format("%H:%M:%S").to_string();
        let time_entry = crate::model::Entry {
            id: String::from("time-entry"),
            title: formatted_time,
            action: String::from(""),
            meta: String::from("Clock Time"),
        };
        // TODO: remove all_entries
        self.plugin.entries.push(time_entry);

        let formatted_date = date.format("%A, %_d. %B %Y").to_string();
        let date_entry = crate::model::Entry {
            id: String::from("date"),
            title: formatted_date,
            action: String::from(""),
            meta: String::from("Clock Date"),
        };
        self.plugin.entries.push(date_entry);

        return self.search(self.last_query.clone());
    }

    fn search(
        &mut self,
        query: String,
    ) -> Result<(), iced::futures::channel::mpsc::TrySendError<crate::Message>> {
        self.last_query = query.clone();

        let filtered_entries = crate::plugin::utils::search(self.plugin.entries.clone(), &query);

        self.plugin_channel_out
            .try_send(crate::Message::Clear(self.plugin.id.clone()))?;

        for entry in filtered_entries {
            self.plugin_channel_out
                .try_send(crate::Message::AppendEntry(self.plugin.id.clone(), entry))?;
        }

        return Ok(());
    }
}
