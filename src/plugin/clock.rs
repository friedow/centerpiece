use iced::futures::sink::SinkExt;
use iced::futures::{FutureExt, StreamExt};

pub fn from(
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

pub async fn update(app_channel_sender: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        plugin_channel_receiver: &mut iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>) {
        let timer = async_std::task::sleep(std::time::Duration::from_secs(1)).fuse();
        let plugin_request = plugin_channel_receiver.select_next_some().fuse();

        iced::futures::pin_mut!(timer, plugin_request);

        let input = iced::futures::select! {
            _ = timer => crate::model::PluginRequest::None,
            plugin_request_message = plugin_request => plugin_request_message,
        };

        match input {
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
