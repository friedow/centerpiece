use async_trait::async_trait;
use iced::futures::sink::SinkExt;
use iced::futures::StreamExt;

pub struct ClockPlugin {
    is_initial_run: bool,
    last_query: String,
    all_entries: Vec<crate::model::EntryModel>,
}

impl ClockPlugin {
    pub fn new() -> ClockPlugin {
        return ClockPlugin {
            is_initial_run: true,
            last_query: String::new(),
            all_entries: vec![],
        };
    }

    fn update_entries(
        &mut self,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) {
        self.all_entries.clear();
        let date = chrono::Local::now();

        let formatted_time = date.format("%H:%M:%S").to_string();
        let time_entry = crate::model::EntryModel {
            id: String::from("time-entry"),
            title: formatted_time,
            action: String::from("open"),
        };
        self.all_entries.push(time_entry.clone());

        let formatted_date = date.format("%A, %_d. %B %Y").to_string();
        let date_entry = crate::model::EntryModel {
            id: String::from("date"),
            title: formatted_date,
            action: String::from("open"),
        };
        self.all_entries.push(date_entry.clone());

        self.search(plugin_channel_out, self.last_query.clone());
    }

    async fn search(
        &mut self,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        query: String,
    ) {
        self.last_query = query;

        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

        let filtered_entries = self
            .all_entries
            .iter()
            .filter_map(|entry| {
                let match_result = matcher.fuzzy_indices(entry.title, query);
                if match_result.is_none() {
                    return None;
                }
                let (score, _) = match_result.unwrap();
                return Some((score, entry));
            })
            .collect::<Vec<(i32, &crate::model::EntryModel)>>();

        filtered_entries.sort_by_cached_key(|(score, entry)| score);

        // TODO: it may be more performant to convert this into a send_all
        let _ = plugin_channel_out
            .send(crate::Message::Clear(String::from("clock")))
            .await;

        for (score, entry) in filtered_entries {
            let _ = plugin_channel_out
                .send(crate::Message::AppendEntry(
                    String::from("clock"),
                    entry.clone(),
                ))
                .await;
        }
    }
}

impl crate::plugin::CreatePluginModel for ClockPlugin {
    fn plugin_model_from(
        &mut self,
        app_channel_out: iced::futures::channel::mpsc::Sender<crate::plugin::PluginRequest>,
    ) -> crate::model::PluginModel {
        return crate::model::PluginModel {
            id: String::from("clock"),
            priority: 0,
            title: String::from("󰅐 Clock"),
            app_channel_out,
            entries: vec![],
        };
    }
}

#[async_trait]
impl crate::plugin::Update for ClockPlugin {
    async fn update(
        &mut self,
        plugin_channel_in: &mut iced::futures::channel::mpsc::Receiver<
            crate::plugin::PluginRequest,
        >,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) {
        let plugin_request = if self.is_initial_run {
            self.is_initial_run = false;
            crate::plugin::PluginRequest::Timeout
        } else {
            let plugin_request_future = plugin_channel_in.select_next_some();
            let plugin_request = async_std::future::timeout(
                std::time::Duration::from_secs(1),
                plugin_request_future,
            )
            .await
            .unwrap_or(crate::plugin::PluginRequest::Timeout);
            plugin_request
        };

        match plugin_request {
            crate::plugin::PluginRequest::Search(query) => {
                self.search(plugin_channel_out, query).await
            }
            crate::plugin::PluginRequest::Timeout => self.update_entries(plugin_channel_out),
        }
    }
}
