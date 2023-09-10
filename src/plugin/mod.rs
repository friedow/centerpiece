use async_trait::async_trait;
use iced::futures::sink::SinkExt;

pub mod clock;

pub enum PluginRequest {
    Search(String),
    Timeout,
}

pub trait CreatePluginModel {
    fn plugin_model_from(
        &mut self,
        channel_in: iced::futures::channel::mpsc::Sender<PluginRequest>,
    ) -> crate::model::PluginModel;
}

#[async_trait]
pub trait Update {
    async fn update(
        &mut self,
        plugin_channel_in: &mut iced::futures::channel::mpsc::Receiver<
            PluginRequest,
        >,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    );
}

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
        let mut plugin = match self {
            Plugin::Clock => crate::plugin::clock::ClockPlugin::new(),
        };

        let mut plugin_channel_in = self.initialize(&mut plugin, &mut plugin_channel_out).await;
        loop {
            self.update(&mut plugin, &mut plugin_channel_in, &mut plugin_channel_out).await;
        }
    }

    async fn initialize(
        &self,
        plugin: &mut impl CreatePluginModel,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> iced::futures::channel::mpsc::Receiver<PluginRequest> {
        let (app_channel_out, plugin_channel_in) =
            iced::futures::channel::mpsc::channel(100);

        let plugin_information = plugin.plugin_model_from(app_channel_out);

        // Send the sender back to the application
        let _ = plugin_channel_out
            .send(crate::Message::RegisterPlugin(plugin_information))
            .await;

        return plugin_channel_in;
    }

    async fn update(
        &self,
        plugin: &mut impl Update,
        plugin_channel_in: &mut iced::futures::channel::mpsc::Receiver<PluginRequest>,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) {
        plugin.update(plugin_channel_in, plugin_channel_out).await;
    }
}
