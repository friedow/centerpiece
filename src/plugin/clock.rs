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
                            // channel: sender,
                            entries: vec![],
                        };

                        // Send the sender back to the application
                        let _ = output.send(crate::Message::RegisterPlugin(plugin, sender)).await;

                        // We are ready to receive messages
                        state = State::Ready(receiver);
                    }
                    State::Ready(receiver) => {
                        // Read next input sent from `Application`
                        // todo: doesnt work
                        // let mut fused_receiver = receiver.by_ref().fuse();
                        // iced::futures::select! {

                        let _ = receiver.select_next_some().await;
                        let _ = output
                            .send(crate::Message::AppendEntry(
                                String::from("clock"),
                                crate::model::Entry {
                                    id: String::from("clock-item-1"),
                                    title: String::from("Item 1"),
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

                        // match input {
                        //     PluginRequest::Search(query) => {

                        // Do some async work...

                        // Finally, we can optionally produce a message to tell the
                        // `Application` the work is done
                        // let _ = output.send(crate::Message::AppendEntry(String::from("clock"), crate::model::Entry {
                        //     id: String::from("clock-item-1"),
                        //     title: String::from("Item 1"),
                        //     action: String::from("open"),
                        // })).await;

                        // let _ = output.send(crate::Message::AppendEntry(String::from("clock"), crate::model::Entry {
                        //     id: String::from("clock-item-2"),
                        //     title: String::from("Item 2"),
                        //     action: String::from("open"),
                        // })).await;
                        //     }
                        // }
                    }
                }
            }
        },
    )
}
