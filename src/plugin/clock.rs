use async_trait::async_trait;
use iced::futures::sink::SinkExt;
use iced::futures::{StreamExt};

pub struct ClockPlugin {
    is_initial_run: bool,
}

pub trait CreatePlugin {
    fn pluginFrom(
        &mut self,
        channel_in: iced::futures::channel::mpsc::Sender<crate::model::PluginRequest>,
    ) -> crate::model::Plugin;
}

#[async_trait]
pub trait Update {
    async fn update(
        &mut self,
        app_channel_sender: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        plugin_channel_receiver: &mut iced::futures::channel::mpsc::Receiver<
            crate::model::PluginRequest,
        >,
    );
}

impl ClockPlugin {
    pub fn new() -> ClockPlugin {
        return ClockPlugin {
            is_initial_run: true,
        };
    }
}

impl CreatePlugin for ClockPlugin {
    fn pluginFrom(
        &mut self,
        channel_in: iced::futures::channel::mpsc::Sender<crate::model::PluginRequest>,
    ) -> crate::model::Plugin {
        return crate::model::Plugin {
            id: String::from("clock"),
            priority: 0,
            title: String::from("󰅐 Clock"),
            channel: channel_in,
            entries: vec![],
        };
    }
}

#[async_trait]
impl Update for ClockPlugin {
    async fn update(
        &mut self,
        app_channel_sender: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        plugin_channel_receiver: &mut iced::futures::channel::mpsc::Receiver<
            crate::model::PluginRequest,
        >,
    ) {
        let plugin_request = if self.is_initial_run {
            self.is_initial_run = false;
            crate::model::PluginRequest::None
        } else {
            let plugin_request_future = plugin_channel_receiver.select_next_some();
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
                let _ = app_channel_sender
                    .send(crate::Message::Clear(String::from("clock")))
                    .await;

                let date = chrono::Local::now();

                let formatted_time = date.format("%H:%M:%S").to_string();
                let _ = app_channel_sender
                    .send(crate::Message::AppendEntry(
                        String::from("clock"),
                        crate::model::Entry {
                            id: String::from("time-entry"),
                            title: formatted_time,
                            action: String::from("open"),
                        },
                    ))
                    .await;

                let formatted_date = date.format("%A, %_d. %B %Y").to_string();
                let _ = app_channel_sender
                    .send(crate::Message::AppendEntry(
                        String::from("clock"),
                        crate::model::Entry {
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
