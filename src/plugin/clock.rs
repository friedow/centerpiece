use iced::futures::sink::SinkExt;
use iced::futures::{FutureExt, StreamExt};

pub enum PluginRequest {
    Search(String),
    None,
}

enum State {
    Starting,
    Ready(iced::futures::channel::mpsc::Receiver<PluginRequest>),
}

pub fn spawn() -> iced::Subscription<crate::Message> {
    struct SomeWorker;

    return iced::subscription::channel(
        std::any::TypeId::of::<SomeWorker>(),
        100,
        |app_channel_sender| async { main(app_channel_sender).await },
    );
}

async fn main(mut app_channel_sender: iced::futures::channel::mpsc::Sender<crate::Message>) -> ! {
    let mut state = State::Starting;

    loop {
        match &mut state {
            State::Starting => {
                initialize(&mut app_channel_sender, &mut state).await;
            }
            State::Ready(plugin_channel_receiver) => {
                update(&mut app_channel_sender, plugin_channel_receiver).await;
            }
        }
    }
}

async fn initialize(
    app_channel_sender: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    state: &mut State,
) {
    let (plugin_channel_sender, plugin_channel_receiver) =
        iced::futures::channel::mpsc::channel(100);

    let plugin = crate::model::Plugin {
        id: String::from("clock"),
        priority: 0,
        title: String::from("󰅐 Clock"),
        channel: plugin_channel_sender,
        entries: vec![],
    };

    // Send the sender back to the application
    let _ = app_channel_sender
        .send(crate::Message::RegisterPlugin(plugin))
        .await;

    // We are ready to receive messages
    *state = State::Ready(plugin_channel_receiver);
}

async fn update(
    app_channel_sender: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_receiver: &mut iced::futures::channel::mpsc::Receiver<PluginRequest>,
) {
    let timer = async_std::task::sleep(std::time::Duration::from_secs(1)).fuse();
    let plugin_request = plugin_channel_receiver.select_next_some().fuse();
    // future

    iced::futures::pin_mut!(timer, plugin_request);

    let input = iced::futures::select! {
        _ = timer => PluginRequest::None,
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


