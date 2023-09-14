use std::format;

use fuzzy_matcher::FuzzyMatcher;
use iced::futures::sink::SinkExt;
use iced::futures::StreamExt;

pub struct ClockPlugin {
    plugin: crate::model::PluginModel,
    is_initial_run: bool,
    last_query: String,
    all_entries: Vec<crate::model::EntryModel>,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::plugin::PluginRequest>,
}

// TODO: most Strings can probable be converted to &str
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
            is_initial_run: true,
            last_query: String::new(),
            all_entries: vec![],
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::PluginModel {
                id: String::from("clock"),
                priority: 0,
                title: String::from("󰅐 Clock"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    async fn main(&mut self) -> ! {
        self.register_plugin().await;

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
        let plugin_request = if self.is_initial_run {
            self.is_initial_run = false;
            crate::plugin::PluginRequest::Timeout
        } else {
            let plugin_request_future = self.plugin_channel_in.select_next_some();
            let plugin_request = async_std::future::timeout(
                std::time::Duration::from_secs(1),
                plugin_request_future,
            )
            .await
            .unwrap_or(crate::plugin::PluginRequest::Timeout);
            plugin_request
        };

        match plugin_request {
            crate::plugin::PluginRequest::Search(query) => self.search(query).await,
            crate::plugin::PluginRequest::Timeout => self.update_entries().await,
        }
    }

    async fn update_entries(
        &mut self,
    ) {
        self.all_entries.clear();
        let date = chrono::Local::now();

        let formatted_time = date.format("%H:%M:%S").to_string();
        let time_entry = crate::model::EntryModel {
            id: String::from("time-entry"),
            title: formatted_time,
            action: String::from("open"),
            meta: String::from("Clock Time"),
        };
        self.all_entries.push(time_entry.clone());

        let formatted_date = date.format("%A, %_d. %B %Y").to_string();
        let date_entry = crate::model::EntryModel {
            id: String::from("date"),
            title: formatted_date,
            action: String::from("open"),
            meta: String::from("Clock Date"),
        };
        self.all_entries.push(date_entry.clone());

        self.search(self.last_query.clone())
            .await;
    }

    async fn search(
        &mut self,
        query: String,
    ) {
        self.last_query = query.clone();

        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

        let mut filtered_entries = self
            .all_entries
            .iter()
            .filter_map(|entry| {
                let keywords = format!("{} {}", entry.title, entry.meta);
                let match_result = matcher.fuzzy_indices(&keywords, &query);
                if match_result.is_none() {
                    return None;
                }
                let (score, _) = match_result.unwrap();
                return Some((score, entry));
            })
            .collect::<Vec<(i64, &crate::model::EntryModel)>>();

        filtered_entries.sort_by_cached_key(|(score, _)| score.clone());

        // TODO: it may be more performant to convert this into a send_all
        let _ = self.plugin_channel_out
            .send(crate::Message::Clear(String::from("clock")))
            .await;

        for (_, entry) in filtered_entries {
            let _ = self.plugin_channel_out
                .send(crate::Message::AppendEntry(
                    String::from("clock"),
                    entry.clone(),
                ))
                .await;
        }
    }
}
