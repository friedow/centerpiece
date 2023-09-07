use iced::futures::sink::SinkExt;

pub mod clock;

pub enum Plugin {
    Clock,
}

impl Plugin {
    pub fn spawn(&'static self) -> iced::Subscription<crate::Message> {
        struct SomeWorker;

        return iced::subscription::channel(
            std::any::TypeId::of::<SomeWorker>(),
            100,
            |app_channel_sender| async { self.main(app_channel_sender).await },
        );
    }

    async fn main(&self, mut app_channel_sender: iced::futures::channel::mpsc::Sender<crate::Message>) -> ! {
        let mut state = crate::model::PluginState::Starting;

        loop {
            match &mut state {
                crate::model::PluginState::Starting => {
                    self.initialize(&mut app_channel_sender, &mut state).await;
                }
                crate::model::PluginState::Ready(plugin_channel_receiver) => {
                    self.update(&mut app_channel_sender, plugin_channel_receiver).await;
                }
            }
        }
    }

    async fn initialize(
        &self,
        app_channel_sender: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        state: &mut crate::model::PluginState,
    ) {
        let (plugin_channel_sender, plugin_channel_receiver) =
            iced::futures::channel::mpsc::channel(100);

        let plugin = match self {
            Plugin::Clock => crate::plugin::clock::from(plugin_channel_sender)
        };

        // Send the sender back to the application
        let _ = app_channel_sender
            .send(crate::Message::RegisterPlugin(plugin))
            .await;

        // We are ready to receive messages
        *state = crate::model::PluginState::Ready(plugin_channel_receiver);
    }

    async fn update(
        &self,
        app_channel_sender: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        plugin_channel_receiver: &mut iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
    ) {
        match self {
            Plugin::Clock => crate::plugin::clock::update(app_channel_sender, plugin_channel_receiver).await,
        }
    }
}
