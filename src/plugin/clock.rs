use async_trait::async_trait;
use iced::futures::sink::SinkExt;
use iced::futures::StreamExt;

pub struct ClockPlugin {
    is_initial_run: bool,
}

impl ClockPlugin {
    pub fn new() -> ClockPlugin {
        return ClockPlugin {
            is_initial_run: true,
        };
    }
}

impl crate::plugin::CreatePluginModel for ClockPlugin {
    fn plugin_model_from(
        &mut self,
        app_channel_out: iced::futures::channel::mpsc::Sender<crate::model::PluginRequest>,
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
            crate::model::PluginRequest,
        >,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) {
        let plugin_request = if self.is_initial_run {
            self.is_initial_run = false;
            crate::model::PluginRequest::None
        } else {
            let plugin_request_future = plugin_channel_in.select_next_some();
            let plugin_request = async_std::future::timeout(
                std::time::Duration::from_secs(1),
                plugin_request_future,
            )
            .await
            .unwrap_or(crate::model::PluginRequest::None);
            plugin_request
        };

        match plugin_request {
            _ => {
                let _ = plugin_channel_out
                    .send(crate::Message::Clear(String::from("clock")))
                    .await;

                let date = chrono::Local::now();

                let formatted_time = date.format("%H:%M:%S").to_string();
                let _ = plugin_channel_out
                    .send(crate::Message::AppendEntry(
                        String::from("clock"),
                        crate::model::EntryModel {
                            id: String::from("time-entry"),
                            title: formatted_time,
                            action: String::from("open"),
                        },
                    ))
                    .await;

                let formatted_date = date.format("%A, %_d. %B %Y").to_string();
                let _ = plugin_channel_out
                    .send(crate::Message::AppendEntry(
                        String::from("clock"),
                        crate::model::EntryModel {
                            id: String::from("date"),
                            title: formatted_date,
                            action: String::from("open"),
                        },
                    ))
                    .await;
            }
        }
    }
}
