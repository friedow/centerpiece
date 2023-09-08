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
            |plugin_channel_out| async { self.main(plugin_channel_out).await },
        );
    }

    async fn main(&self, mut plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>) -> ! {
        let mut state = crate::model::PluginState::Starting;

        let mut plugin = match self {
            Plugin::Clock => crate::plugin::clock::ClockPlugin::new(),
        };

        loop {
            match &mut state {
                crate::model::PluginState::Starting => {
                    self.initialize(&mut plugin, &mut plugin_channel_out, &mut state).await;
                }
                crate::model::PluginState::Ready(plugin_channel_in) => {
                    self.update(&mut plugin, &mut plugin_channel_out, plugin_channel_in).await;
                }
            }
        }
    }

    async fn initialize(
        &self,
        plugin: &mut impl crate::plugin::clock::CreatePlugin,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        state: &mut crate::model::PluginState,
    ) {
        let (app_channel_out, plugin_channel_in) =
            iced::futures::channel::mpsc::channel(100);

        let plugin_information = plugin.plugin_information_from(app_channel_out);

        // Send the sender back to the application
        let _ = plugin_channel_out
            .send(crate::Message::RegisterPlugin(plugin_information))
            .await;

        // We are ready to receive messages
        *state = crate::model::PluginState::Ready(plugin_channel_in);
    }

    async fn update(
        &self,
        plugin: &mut impl crate::plugin::clock::Update,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        plugin_channel_in: &mut iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
    ) {
        plugin.update(plugin_channel_out, plugin_channel_in).await;
    }
}
