use iced::futures::sink::SinkExt;
use iced::futures::StreamExt;

pub enum PluginRequest {
    Search(String),
}

enum State {
    Starting,
    Ready(iced::futures::channel::mpsc::Receiver<PluginRequest>),
}

pub fn spawn() -> iced::Subscription<crate::Message> {
    struct SomeWorker;

    iced::subscription::channel(
        std::any::TypeId::of::<SomeWorker>(),
        100,
        |mut output| async move {
            let mut state = State::Starting;

            loop {
                match &mut state {
                    State::Starting => {
                        let (sender, receiver) = iced::futures::channel::mpsc::channel(100);

                        let plugin = crate::model::Plugin {
                            id: String::from("clock"),
                            priority: 0,
                            title: String::from("󰅐 Clock"),
                            channel: sender,
                            entries: vec![],
                        };

                        // Send the sender back to the application
                        let _ = output.send(crate::Message::RegisterPlugin(plugin)).await;

                        // We are ready to receive messages
                        state = State::Ready(receiver);
                    }
                    State::Ready(receiver) => {
                        let input = receiver.select_next_some().await;

                        match input {
                            PluginRequest::Search(query) => {
                                let _ = output
                                    .send(crate::Message::AppendEntry(
                                        String::from("clock"),
                                        crate::model::Entry {
                                            id: String::from("clock-item-1"),
                                            title: String::from(query),
                                            action: String::from("open"),
                                        },
                                    ))
                                    .await;

                                let _ = output
                                    .send(crate::Message::AppendEntry(
                                        String::from("clock"),
                                        crate::model::Entry {
                                            id: String::from("clock-item-2"),
                                            title: String::from("Item 2"),
                                            action: String::from("open"),
                                        },
                                    ))
                                    .await;
                            }
                        }
                    }
                }
            }
        },
    )
}
