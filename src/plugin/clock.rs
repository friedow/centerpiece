use iced::futures::sink::SinkExt;
use iced::futures::StreamExt;

pub struct ClockPlugin {
    plugin: crate::model::Plugin,
    last_query: String,
    all_entries: Vec<crate::model::Entry>,
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
            all_entries: vec![],
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("clock"),
                priority: 10,
                title: String::from(" Clock"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    async fn main(&mut self) -> ! {
        self.register_plugin().await;
        self.update_entries().await;

        loop {
            self.update().await;
        }
    }

    async fn register_plugin(&mut self) {
        let _ = self
            .plugin_channel_out
            .send(crate::Message::RegisterPlugin(self.plugin.clone()))
            .await;
    }

    async fn update(&mut self) {
        let plugin_request_future = self.plugin_channel_in.select_next_some();
        let plugin_request =
            async_std::future::timeout(std::time::Duration::from_secs(1), plugin_request_future)
                .await
                .unwrap_or(crate::model::PluginRequest::Timeout);

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(query),
            crate::model::PluginRequest::Timeout => self.update_entries().await,
            crate::model::PluginRequest::Activate(_) => (),
        }
    }

    async fn update_entries(&mut self) {
        self.all_entries.clear();
        let date = chrono::Local::now();

        let formatted_time = date.format("%H:%M:%S").to_string();
        let time_entry = crate::model::Entry {
            id: String::from("time-entry"),
            title: formatted_time,
            action: String::from("open"),
            meta: String::from("Clock Time"),
        };
        self.all_entries.push(time_entry.clone());

        let formatted_date = date.format("%A, %_d. %B %Y").to_string();
        let date_entry = crate::model::Entry {
            id: String::from("date"),
            title: formatted_date,
            action: String::from("open"),
            meta: String::from("Clock Date"),
        };
        self.all_entries.push(date_entry.clone());

        self.search(self.last_query.clone());
    }

    fn search(&mut self, query: String) {
        self.last_query = query.clone();

        let filtered_entries = crate::plugin::utils::search(self.all_entries.clone(), &query);

        self.plugin_channel_out
            .try_send(crate::Message::Clear(self.plugin.id.clone()))
            .ok();

        for entry in filtered_entries {
            self.plugin_channel_out
                .try_send(crate::Message::AppendEntry(self.plugin.id.clone(), entry))
                .ok();
        }
    }
}
